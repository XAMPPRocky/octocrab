use super::merger::build_node_envelope;
use super::tree::QueryNode;
use crate::client::GithubClient;
use crate::registry::{ParentContext, get_entity};
use serde_json::Value;
use web_time::Instant;

#[derive(Clone, Debug)]
pub struct ExecutionOutcome {
    pub root_result: Value,
    pub total_duration_ms: u128,
    pub has_errors: bool,
    pub rate_limit_remaining: Option<u64>,
    pub rate_limit_limit: Option<u64>,
}

pub async fn execute_tree(root: &QueryNode, client: &GithubClient) -> ExecutionOutcome {
    let start = Instant::now();
    let mut initial_context = ParentContext::new();

    let (root_envelope, has_errors) =
        execute_node_recursive(root, client, &mut initial_context).await;
    let total_duration_ms = start.elapsed().as_millis();

    let remaining = client.rate_limit_remaining();
    let limit = client.rate_limit_limit();

    ExecutionOutcome {
        root_result: root_envelope,
        total_duration_ms,
        has_errors,
        rate_limit_remaining: remaining,
        rate_limit_limit: limit,
    }
}

#[async_recursion::async_recursion(?Send)]
async fn execute_node_recursive(
    node: &QueryNode,
    client: &GithubClient,
    inherited_context: &mut ParentContext,
) -> (Value, bool) {
    let start = Instant::now();
    let entity_opt = get_entity(&node.entity_id);

    let (mut data, status, error, child_context, mut has_errors) = match entity_opt.as_ref() {
        Some(entity) => match entity
            .execute(client, &node.params, inherited_context)
            .await
        {
            Ok(val) => {
                let ctx = entity.extract_context(&node.params, &val);
                (Some(val), "success", None, ctx, false)
            }
            Err(e) => (None, "error", Some(e), ParentContext::new(), true),
        },
        None => (
            None,
            "error",
            Some(format!("Unknown entity type: {}", node.entity_id)),
            ParentContext::new(),
            true,
        ),
    };

    let mut merged_context = inherited_context.clone();
    merged_context.merge(&child_context);

    // If parent is a collection (e.g. org_repos, user_repos, repo_issues)
    // and has child nodes, execute children for each item and attach directly
    // to each item's defined target_field
    let collection_items = if entity_opt
        .as_ref()
        .is_some_and(|e| e.is_collection() && !node.children.is_empty())
    {
        data.as_mut().and_then(|d| match d {
            Value::Array(items) => entity_opt.as_ref().map(|e| (&**e, items)),
            _ => None,
        })
    } else {
        None
    };

    if let Some((entity, items)) = collection_items {
        for item in items.iter_mut() {
            let item_context_patch =
                entity.extract_item_context(&node.params, &merged_context, item);
            let mut item_context = merged_context.clone();
            item_context.merge(&item_context_patch);

            if let Value::Object(item_obj) = item {
                for child in &node.children {
                    let (child_envelope, child_has_err) =
                        execute_node_recursive(child, client, &mut item_context).await;
                    if child_has_err {
                        has_errors = true;
                    }
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());

                    // Directly attach child data to item[target_field]
                    let child_data = child_envelope
                        .get("data")
                        .cloned()
                        .unwrap_or(child_envelope);
                    item_obj.insert(target_field.to_string(), child_data);
                }
            }
        }

        let duration_ms = start.elapsed().as_millis();
        let final_envelope = build_node_envelope(
            &node.entity_id,
            status,
            duration_ms,
            error,
            data,
            serde_json::Map::new(),
        );
        return (final_envelope, has_errors);
    }

    // For non-collection parent or single entities (e.g. org, repo, issue, pull),
    // execute children and attach directly to the parent object's target_field
    let mut children_map = serde_json::Map::new();
    for child in &node.children {
        let (child_envelope, child_has_err) =
            execute_node_recursive(child, client, &mut merged_context).await;
        if child_has_err {
            has_errors = true;
        }

        let child_def = get_entity(&child.entity_id);
        let target_field = child_def
            .as_ref()
            .map_or(child.entity_id.as_str(), |d| d.target_field());

        let child_data = child_envelope
            .get("data")
            .cloned()
            .unwrap_or_else(|| child_envelope.clone());

        if let Some(Value::Object(parent_obj)) = data.as_mut() {
            parent_obj.insert(target_field.to_string(), child_data);
        }

        children_map.insert(child.entity_id.clone(), child_envelope);
    }

    let duration_ms = start.elapsed().as_millis();
    let final_envelope = build_node_envelope(
        &node.entity_id,
        status,
        duration_ms,
        error,
        data,
        children_map,
    );

    (final_envelope, has_errors)
}
