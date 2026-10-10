use super::tree::QueryNode;
use crate::registry::{ParentContext, get_entity};

/// Generates reproducible, stand-alone Rust code using `octocrab`
/// that queries the exact same hierarchical data structure configured in the playground.
pub fn generate_rust_code(root: &QueryNode, with_auth: bool) -> String {
    let mut code = String::new();

    code.push_str("//! Auto-generated Octocrab query code\n");
    code.push_str(
        "//! This file demonstrates how to reproduce the playground query using octocrab.\n",
    );
    code.push_str("//!\n");
    code.push_str("//! Dependencies required in Cargo.toml:\n");
    code.push_str("//! ```toml\n");
    code.push_str("//! [dependencies]\n");
    code.push_str("//! octocrab = \"0.54\"\n");
    code.push_str("//! tokio = { version = \"1\", features = [\"full\"] }\n");
    code.push_str("//! serde_json = \"1.0\"\n");
    code.push_str("//! ```\n\n");

    code.push_str("use serde_json::json;\n");
    code.push_str("use std::error::Error;\n\n");

    code.push_str("#[tokio::main]\n");
    code.push_str("async fn main() -> Result<(), Box<dyn Error>> {\n");

    if with_auth {
        code.push_str("    // Initialize Octocrab client with personal access token\n");
        code.push_str("    let token = std::env::var(\"GITHUB_TOKEN\").unwrap_or_else(|_| \"your_token_here\".to_string());\n");
        code.push_str("    let octocrab = octocrab::Octocrab::builder()\n");
        code.push_str("        .personal_token(token)\n");
        code.push_str("        .build()?;\n\n");
    } else {
        code.push_str("    // Initialize default unauthenticated Octocrab client\n");
        code.push_str("    let octocrab = octocrab::instance();\n\n");
    }

    let mut context = ParentContext::new();
    let mut aggregators = Vec::new();
    generate_node_code(root, &mut context, 1, &mut code, &mut aggregators);

    code.push_str("\n    // Output formatted merged JSON result\n");
    code.push_str("    println!(\"{}\", serde_json::to_string_pretty(&merged_result)?);\n");
    code.push_str("    Ok(())\n");
    code.push_str("}\n");

    code
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

fn format_str_arg(val: &str) -> String {
    if val.starts_with('&')
        || val.starts_with('"')
        || val.contains("repo_item")
        || val.contains("pull_item")
        || val.contains("issue_item")
    {
        val.to_string()
    } else if val == "owner" || val == "repo_name" || val == "username" {
        format!("&{val}")
    } else {
        format!("\"{val}\"")
    }
}

fn get_str_param(node: &QueryNode, ctx: &ParentContext, key: &str, default_val: &str) -> String {
    node.params
        .get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .or_else(|| ctx.get(key).map(|s| s.to_string()))
        .unwrap_or_else(|| default_val.to_string())
}

fn get_i64_param(node: &QueryNode, key: &str, default_val: i64) -> i64 {
    node.params
        .get(key)
        .and_then(|v| v.as_i64())
        .unwrap_or(default_val)
}

fn generate_node_code(
    node: &QueryNode,
    ctx: &mut ParentContext,
    indent_level: usize,
    code: &mut String,
    aggregators: &mut Vec<String>,
) {
    let ind = indent(indent_level);

    match node.entity_id.as_str() {
        "org" => {
            let org = get_str_param(node, ctx, "org", "rust-lang");
            ctx.set("org", org.clone());

            code.push_str(&format!("{ind}// Fetch organization details: {org}\n"));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.orgs(\"{org}\").get().await?;\n"
            ));
            code.push_str(&format!("{ind}let mut merged_result = json!(root_data);\n"));
            aggregators.push("merged_result".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "org_repos" => {
            let org = get_str_param(node, ctx, "org", "rust-lang");
            let r#type = get_str_param(node, ctx, "type", "all");
            let sort = get_str_param(node, ctx, "sort", "updated");
            let per_page = get_i64_param(node, "per_page", 10);

            if node.children.is_empty() {
                code.push_str(&format!(
                    "\n{ind}// Fetch repositories for organization: {org}\n"
                ));
                code.push_str(&format!(
                    "{ind}let org_repos_data = octocrab.orgs(\"{org}\")\n"
                ));
                code.push_str(&format!("{ind}    .list_repos()\n"));
                if r#type != "all" {
                    code.push_str(&format!(
                        "{ind}    .repo_type(octocrab::params::repos::Type::{})\n",
                        capitalize(&r#type)
                    ));
                }
                if sort != "updated" {
                    code.push_str(&format!(
                        "{ind}    .sort(octocrab::params::repos::Sort::{})\n",
                        capitalize(&sort)
                    ));
                }
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                let target_field =
                    get_entity(&node.entity_id).map_or("repositories", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(org_repos_data);\n"
                ));
            } else {
                code.push_str(&format!(
                    "\n{ind}// Fetch repositories for organization: {org}\n"
                ));
                code.push_str(&format!(
                    "{ind}let org_repos_page = octocrab.orgs(\"{org}\")\n"
                ));
                code.push_str(&format!("{ind}    .list_repos()\n"));
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                code.push_str(&format!(
                    "\n{ind}// For each repository, fetch child data and attach to parent field\n"
                ));
                code.push_str(&format!("{ind}let mut org_repos_data = Vec::new();\n"));
                code.push_str(&format!("{ind}for repo_item in org_repos_page.items {{\n"));
                let inner_ind = format!("{ind}    ");
                code.push_str(&format!(
                    "{inner_ind}let repo_name = repo_item.name.clone();\n"
                ));
                code.push_str(&format!(
                    "{inner_ind}let owner = repo_item.owner.as_ref().map(|o| o.login.clone()).unwrap_or_else(|| \"{org}\".to_string());\n"
                ));

                let mut child_ctx = ctx.clone();
                child_ctx.set("owner", "owner");
                child_ctx.set("repo", "repo_name");

                let mut child_aggs = Vec::new();
                for child in &node.children {
                    generate_node_code(
                        child,
                        &mut child_ctx,
                        indent_level + 1,
                        code,
                        &mut child_aggs,
                    );
                }

                code.push_str(&format!(
                    "\n{inner_ind}let mut repo_val = json!(repo_item);\n"
                ));
                for child in &node.children {
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());
                    let child_var = format!("{}_data", child.entity_id);
                    code.push_str(&format!(
                        "{inner_ind}repo_val[\"{}\"] = json!({});\n",
                        target_field, child_var
                    ));
                }
                code.push_str(&format!("{inner_ind}org_repos_data.push(repo_val);\n"));
                code.push_str(&format!("{ind}}}\n"));
                let target_field =
                    get_entity(&node.entity_id).map_or("repositories", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(org_repos_data);\n"
                ));
            }
        }
        "org_members" => {
            let org = get_str_param(node, ctx, "org", "rust-lang");
            let per_page = get_i64_param(node, "per_page", 10);

            code.push_str(&format!("\n{ind}// Fetch members of organization: {org}\n"));
            code.push_str(&format!(
                "{ind}let org_members_data = octocrab.orgs(\"{org}\")\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_members()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("members", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(org_members_data);\n"
            ));
        }
        "org_teams" => {
            let org = get_str_param(node, ctx, "org", "rust-lang");
            let per_page = get_i64_param(node, "per_page", 10);

            code.push_str(&format!("\n{ind}// Fetch teams for organization: {org}\n"));
            code.push_str(&format!(
                "{ind}let org_teams_data = octocrab.teams(\"{org}\")\n"
            ));
            code.push_str(&format!(
                "{ind}    .list()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("teams", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(org_teams_data);\n"
            ));
        }
        "repo" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            ctx.set("owner", owner.clone());
            ctx.set("repo", repo.clone());

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "{ind}// Fetch repository details: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.repos({owner_arg}, {repo_arg}).get().await?;\n"
            ));
            code.push_str(&format!("{ind}let mut merged_result = json!(root_data);\n"));
            aggregators.push("merged_result".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "repo_issues" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let state = get_str_param(node, ctx, "state", "open");
            let per_page = get_i64_param(node, "per_page", 10);

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            if node.children.is_empty() {
                code.push_str(&format!(
                    "\n{ind}// Fetch issues for repository: {owner}/{repo}\n"
                ));
                code.push_str(&format!(
                    "{ind}let repo_issues_data = octocrab.issues({owner_arg}, {repo_arg})\n"
                ));
                code.push_str(&format!("{ind}    .list()\n"));
                if state == "closed" {
                    code.push_str(&format!(
                        "{ind}    .state(octocrab::params::State::Closed)\n"
                    ));
                } else if state == "all" {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::All)\n"));
                } else {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::Open)\n"));
                }
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                let target_field =
                    get_entity(&node.entity_id).map_or("issues", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(repo_issues_data);\n"
                ));
            } else {
                code.push_str(&format!(
                    "\n{ind}// Fetch issues for repository: {owner}/{repo}\n"
                ));
                code.push_str(&format!(
                    "{ind}let repo_issues_page = octocrab.issues({owner_arg}, {repo_arg})\n"
                ));
                code.push_str(&format!("{ind}    .list()\n"));
                if state == "closed" {
                    code.push_str(&format!(
                        "{ind}    .state(octocrab::params::State::Closed)\n"
                    ));
                } else if state == "all" {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::All)\n"));
                } else {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::Open)\n"));
                }
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                code.push_str(&format!(
                    "\n{ind}// For each issue, fetch child data and attach to parent field\n"
                ));
                code.push_str(&format!("{ind}let mut repo_issues_data = Vec::new();\n"));
                code.push_str(&format!(
                    "{ind}for issue_item in repo_issues_page.items {{\n"
                ));
                let inner_ind = format!("{ind}    ");
                code.push_str(&format!(
                    "{inner_ind}let issue_number = issue_item.number;\n"
                ));

                let mut child_ctx = ctx.clone();
                child_ctx.set("issue_number", "issue_number");

                let mut child_aggs = Vec::new();
                for child in &node.children {
                    generate_node_code(
                        child,
                        &mut child_ctx,
                        indent_level + 1,
                        code,
                        &mut child_aggs,
                    );
                }

                code.push_str(&format!(
                    "\n{inner_ind}let mut issue_val = json!(issue_item);\n"
                ));
                for child in &node.children {
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());
                    let child_var = format!("{}_data", child.entity_id);
                    code.push_str(&format!(
                        "{inner_ind}issue_val[\"{}\"] = json!({});\n",
                        target_field, child_var
                    ));
                }
                code.push_str(&format!("{inner_ind}repo_issues_data.push(issue_val);\n"));
                code.push_str(&format!("{ind}}}\n"));
                let target_field =
                    get_entity(&node.entity_id).map_or("issues", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(repo_issues_data);\n"
                ));
            }
        }
        "repo_pulls" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let state = get_str_param(node, ctx, "state", "open");
            let per_page = get_i64_param(node, "per_page", 10);

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            if node.children.is_empty() {
                code.push_str(&format!(
                    "\n{ind}// Fetch pull requests for repository: {owner}/{repo}\n"
                ));
                code.push_str(&format!(
                    "{ind}let repo_pulls_data = octocrab.pulls({owner_arg}, {repo_arg})\n"
                ));
                code.push_str(&format!("{ind}    .list()\n"));
                if state == "closed" {
                    code.push_str(&format!(
                        "{ind}    .state(octocrab::params::State::Closed)\n"
                    ));
                } else if state == "all" {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::All)\n"));
                } else {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::Open)\n"));
                }
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                let target_field =
                    get_entity(&node.entity_id).map_or("pull_requests", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(repo_pulls_data);\n"
                ));
            } else {
                code.push_str(&format!(
                    "\n{ind}// Fetch pull requests for repository: {owner}/{repo}\n"
                ));
                code.push_str(&format!(
                    "{ind}let repo_pulls_page = octocrab.pulls({owner_arg}, {repo_arg})\n"
                ));
                code.push_str(&format!("{ind}    .list()\n"));
                if state == "closed" {
                    code.push_str(&format!(
                        "{ind}    .state(octocrab::params::State::Closed)\n"
                    ));
                } else if state == "all" {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::All)\n"));
                } else {
                    code.push_str(&format!("{ind}    .state(octocrab::params::State::Open)\n"));
                }
                code.push_str(&format!(
                    "{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                code.push_str(&format!(
                    "\n{ind}// For each pull request, fetch child data and attach to parent field\n"
                ));
                code.push_str(&format!("{ind}let mut repo_pulls_data = Vec::new();\n"));
                code.push_str(&format!("{ind}for pull_item in repo_pulls_page.items {{\n"));
                let inner_ind = format!("{ind}    ");
                code.push_str(&format!("{inner_ind}let pull_number = pull_item.number;\n"));

                let mut child_ctx = ctx.clone();
                child_ctx.set("pull_number", "pull_number");

                let mut child_aggs = Vec::new();
                for child in &node.children {
                    generate_node_code(
                        child,
                        &mut child_ctx,
                        indent_level + 1,
                        code,
                        &mut child_aggs,
                    );
                }

                code.push_str(&format!(
                    "\n{inner_ind}let mut pull_val = json!(pull_item);\n"
                ));
                for child in &node.children {
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());
                    let child_var = format!("{}_data", child.entity_id);
                    code.push_str(&format!(
                        "{inner_ind}pull_val[\"{}\"] = json!({});\n",
                        target_field, child_var
                    ));
                }
                code.push_str(&format!("{inner_ind}repo_pulls_data.push(pull_val);\n"));
                code.push_str(&format!("{ind}}}\n"));
                let target_field =
                    get_entity(&node.entity_id).map_or("pull_requests", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(repo_pulls_data);\n"
                ));
            }
        }
        "repo_commits" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch repository commits: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_commits_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_commits()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("commits", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_commits_data);\n"
            ));
        }
        "repo_releases" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch releases for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_releases_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .releases()\n{ind}    .list()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("releases", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_releases_data);\n"
            ));
        }
        "repo_branches" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch branches for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_branches_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_branches()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("branches", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_branches_data);\n"
            ));
        }
        "repo_workflows" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch GitHub Actions workflows for: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_workflows_data = octocrab.workflows({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field =
                get_entity(&node.entity_id).map_or("workflows", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_workflows_data);\n"
            ));
        }
        "repo_tags" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch git tags for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_tags_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_tags()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("tags", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_tags_data);\n"
            ));
        }
        "repo_contributors" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch contributors for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_contributors_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_contributors()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field =
                get_entity(&node.entity_id).map_or("contributors", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_contributors_data);\n"
            ));
        }
        "repo_languages" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch programming languages for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_languages_data = octocrab.repos({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!("{ind}    .list_languages()\n{ind}    .await?;\n"));
            let target_field =
                get_entity(&node.entity_id).map_or("languages", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_languages_data);\n"
            ));
        }
        "repo_labels" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch labels for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_labels_data = octocrab.issues({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_labels_for_repo()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("labels", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_labels_data);\n"
            ));
        }
        "repo_milestones" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "\n{ind}// Fetch milestones for repository: {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let repo_milestones_data = octocrab.issues({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_milestones()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field =
                get_entity(&node.entity_id).map_or("milestones", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(repo_milestones_data);\n"
            ));
        }
        "current_user" => {
            code.push_str(&format!("{ind}// Fetch authenticated user profile\n"));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.current().user().await?;\n"
            ));
            code.push_str(&format!("{ind}let mut merged_result = json!(root_data);\n"));
            aggregators.push("merged_result".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "user" => {
            let username = get_str_param(node, ctx, "username", "torvalds");
            ctx.set("username", username.clone());

            code.push_str(&format!("{ind}// Fetch user profile for: {username}\n"));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.users(\"{username}\").profile().await?;\n"
            ));
            code.push_str(&format!("{ind}let mut merged_result = json!(root_data);\n"));
            aggregators.push("merged_result".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "user_repos" => {
            let username = get_str_param(node, ctx, "username", "torvalds");
            let per_page = get_i64_param(node, "per_page", 10);

            if node.children.is_empty() {
                code.push_str(&format!(
                    "\n{ind}// Fetch public repositories for user: {username}\n"
                ));
                code.push_str(&format!(
                    "{ind}let user_repos_data = octocrab.users(\"{username}\")\n"
                ));
                code.push_str(&format!(
                    "{ind}    .repos()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                let target_field =
                    get_entity(&node.entity_id).map_or("repositories", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(user_repos_data);\n"
                ));
            } else {
                code.push_str(&format!(
                    "\n{ind}// Fetch public repositories for user: {username}\n"
                ));
                code.push_str(&format!(
                    "{ind}let user_repos_page = octocrab.users(\"{username}\")\n"
                ));
                code.push_str(&format!(
                    "{ind}    .repos()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                code.push_str(&format!(
                    "\n{ind}// For each user repository, fetch child data and attach to parent field\n"
                ));
                code.push_str(&format!("{ind}let mut user_repos_data = Vec::new();\n"));
                code.push_str(&format!("{ind}for repo_item in user_repos_page.items {{\n"));
                let inner_ind = format!("{ind}    ");
                code.push_str(&format!(
                    "{inner_ind}let repo_name = repo_item.name.clone();\n"
                ));
                code.push_str(&format!(
                    "{inner_ind}let owner = repo_item.owner.as_ref().map(|o| o.login.clone()).unwrap_or_else(|| \"{username}\".to_string());\n"
                ));

                let mut child_ctx = ctx.clone();
                child_ctx.set("owner", "owner");
                child_ctx.set("repo", "repo_name");

                let mut child_aggs = Vec::new();
                for child in &node.children {
                    generate_node_code(
                        child,
                        &mut child_ctx,
                        indent_level + 1,
                        code,
                        &mut child_aggs,
                    );
                }

                code.push_str(&format!(
                    "\n{inner_ind}let mut repo_val = json!(repo_item);\n"
                ));
                for child in &node.children {
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());
                    let child_var = format!("{}_data", child.entity_id);
                    code.push_str(&format!(
                        "{inner_ind}repo_val[\"{}\"] = json!({});\n",
                        target_field, child_var
                    ));
                }
                code.push_str(&format!("{inner_ind}user_repos_data.push(repo_val);\n"));
                code.push_str(&format!("{ind}}}\n"));
                let target_field =
                    get_entity(&node.entity_id).map_or("repositories", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}merged_result[\"{target_field}\"] = json!(user_repos_data);\n"
                ));
            }
        }
        "user_followers" => {
            let username = get_str_param(node, ctx, "username", "torvalds");
            let per_page = get_i64_param(node, "per_page", 10);

            code.push_str(&format!("\n{ind}// Fetch followers for user: {username}\n"));
            code.push_str(&format!(
                "{ind}let user_followers_data = octocrab.users(\"{username}\")\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_followers()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field =
                get_entity(&node.entity_id).map_or("followers", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(user_followers_data);\n"
            ));
        }
        "user_gists" => {
            let username = get_str_param(node, ctx, "username", "torvalds");
            let per_page = get_i64_param(node, "per_page", 10);

            code.push_str(&format!(
                "\n{ind}// Fetch public gists for user: {username}\n"
            ));
            code.push_str(&format!(
                "{ind}let user_gists_data = octocrab.users(\"{username}\")\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_gists()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("gists", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(user_gists_data);\n"
            ));
        }
        "gist" => {
            let gist_id = get_str_param(node, ctx, "gist_id", "aa5a315d61ae9438b18d");
            ctx.set("gist_id", gist_id.clone());

            code.push_str(&format!("{ind}// Fetch gist details for: {gist_id}\n"));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.gists().get(\"{gist_id}\").await?;\n"
            ));
            code.push_str(&format!("{ind}let mut merged_result = json!(root_data);\n"));
            aggregators.push("merged_result".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "gist_comments" => {
            let gist_id = get_str_param(node, ctx, "gist_id", "aa5a315d61ae9438b18d");
            let per_page = get_i64_param(node, "per_page", 10);
            let gist_id_arg = if gist_id == "gist_id" {
                "gist_id".to_string()
            } else {
                format!("\"{gist_id}\"")
            };

            code.push_str(&format!("\n{ind}// Fetch comments for gist {gist_id}\n"));
            code.push_str(&format!("{ind}let gist_comments_data = octocrab.gists()\n"));
            code.push_str(&format!(
                "{ind}    .comments_for({gist_id_arg})\n{ind}    .per_page({per_page})\n{ind}    .list_comments()\n{ind}    .await?;\n"
            ));
            let target_field = get_entity(&node.entity_id).map_or("comments", |d| d.target_field());
            code.push_str(&format!(
                "{ind}merged_result[\"{target_field}\"] = json!(gist_comments_data);\n"
            ));
        }
        "issue" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let issue_num = get_i64_param(node, "issue_number", 1);
            ctx.set("owner", owner.clone());
            ctx.set("repo", repo.clone());
            ctx.set("issue_number", issue_num.to_string());

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "{ind}// Fetch issue #{issue_num} from {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.issues({owner_arg}, {repo_arg}).get({issue_num}u64).await?;\n"
            ));
            aggregators.push("root_data".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "issue_comments" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let per_page = get_i64_param(node, "per_page", 10);

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!("\n{ind}// Fetch issue comments\n"));
            code.push_str(&format!(
                "{ind}let issue_comments_data = octocrab.issues({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_issue_comments()\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
        }
        "pull" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let pull_num = get_i64_param(node, "pull_number", 1);
            ctx.set("owner", owner.clone());
            ctx.set("repo", repo.clone());
            ctx.set("pull_number", pull_num.to_string());

            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);

            code.push_str(&format!(
                "{ind}// Fetch pull request #{pull_num} from {owner}/{repo}\n"
            ));
            code.push_str(&format!(
                "{ind}let root_data = octocrab.pulls({owner_arg}, {repo_arg}).get({pull_num}u64).await?;\n"
            ));
            aggregators.push("root_data".to_string());

            for child in &node.children {
                generate_node_code(child, ctx, indent_level, code, aggregators);
            }
        }
        "pull_commits" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let pull_num_raw = get_str_param(node, ctx, "pull_number", "1");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);
            let pull_num_arg = if pull_num_raw == "pull_number" {
                "pull_number".to_string()
            } else {
                format!("{pull_num_raw}u64")
            };

            code.push_str(&format!(
                "\n{ind}// Fetch commits in pull request #{pull_num_raw}\n"
            ));
            code.push_str(&format!(
                "{ind}let pull_commits_data = octocrab.pulls({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .pr_commits({pull_num_arg})\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
        }
        "pull_files" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let pull_num_raw = get_str_param(node, ctx, "pull_number", "1");
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);
            let pull_num_arg = if pull_num_raw == "pull_number" {
                "pull_number".to_string()
            } else {
                format!("{pull_num_raw}u64")
            };

            code.push_str(&format!(
                "\n{ind}// Fetch changed files in pull request #{pull_num_raw}\n"
            ));
            code.push_str(&format!(
                "{ind}let pull_files_data = octocrab.pulls({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_files({pull_num_arg})\n{ind}    .await?;\n"
            ));
        }
        "pull_comments" => {
            let owner = get_str_param(node, ctx, "owner", "XAMPPRocky");
            let repo = get_str_param(node, ctx, "repo", "octocrab");
            let pull_num_raw = get_str_param(node, ctx, "pull_number", "1");
            let per_page = get_i64_param(node, "per_page", 10);
            let owner_arg = format_str_arg(&owner);
            let repo_arg = format_str_arg(&repo);
            let pull_num_arg = if pull_num_raw == "pull_number" {
                "Some(pull_number)".to_string()
            } else {
                format!("Some({pull_num_raw}u64)")
            };

            code.push_str(&format!(
                "\n{ind}// Fetch review comments for pull request #{pull_num_raw}\n"
            ));
            code.push_str(&format!(
                "{ind}let pull_comments_data = octocrab.pulls({owner_arg}, {repo_arg})\n"
            ));
            code.push_str(&format!(
                "{ind}    .list_comments({pull_num_arg})\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
        }
        "search_repos" => {
            let q = get_str_param(node, ctx, "q", "rust");
            let sort = get_str_param(node, ctx, "sort", "stars");
            let order = get_str_param(node, ctx, "order", "desc");
            let per_page = get_i64_param(node, "per_page", 10);

            if node.children.is_empty() {
                code.push_str(&format!("{ind}// Search repositories query: \"{q}\"\n"));
                code.push_str(&format!("{ind}let root_data = octocrab.search()\n"));
                code.push_str(&format!(
                    "{ind}    .repositories(\"{q}\")\n{ind}    .sort(\"{sort}\")\n{ind}    .order(\"{order}\")\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                aggregators.push("root_data".to_string());
            } else {
                code.push_str(&format!("{ind}// Search repositories query: \"{q}\"\n"));
                code.push_str(&format!("{ind}let search_page = octocrab.search()\n"));
                code.push_str(&format!(
                    "{ind}    .repositories(\"{q}\")\n{ind}    .sort(\"{sort}\")\n{ind}    .order(\"{order}\")\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
                ));
                code.push_str(&format!(
                    "\n{ind}// For each matching repository, fetch child data and attach to parent field\n"
                ));
                code.push_str(&format!("{ind}let mut search_repos_data = Vec::new();\n"));
                code.push_str(&format!("{ind}for repo_item in search_page.items {{\n"));
                let inner_ind = format!("{ind}    ");
                code.push_str(&format!(
                    "{inner_ind}let repo_name = repo_item.name.clone();\n"
                ));
                code.push_str(&format!(
                    "{inner_ind}let owner = repo_item.owner.as_ref().map(|o| o.login.clone()).unwrap_or_else(|| \"octocat\".to_string());\n"
                ));

                let mut child_ctx = ctx.clone();
                child_ctx.set("owner", "owner");
                child_ctx.set("repo", "repo_name");

                let mut child_aggs = Vec::new();
                for child in &node.children {
                    generate_node_code(
                        child,
                        &mut child_ctx,
                        indent_level + 1,
                        code,
                        &mut child_aggs,
                    );
                }

                code.push_str(&format!(
                    "\n{inner_ind}let mut repo_val = json!(repo_item);\n"
                ));
                for child in &node.children {
                    let child_def = get_entity(&child.entity_id);
                    let target_field = child_def
                        .as_ref()
                        .map_or(child.entity_id.as_str(), |d| d.target_field());
                    let child_var = format!("{}_data", child.entity_id);
                    code.push_str(&format!(
                        "{inner_ind}repo_val[\"{}\"] = json!({});\n",
                        target_field, child_var
                    ));
                }
                code.push_str(&format!("{inner_ind}search_repos_data.push(repo_val);\n"));
                code.push_str(&format!("{ind}}}\n"));
                let target_field =
                    get_entity(&node.entity_id).map_or("repositories", |d| d.target_field());
                code.push_str(&format!(
                    "{ind}let mut merged_result = json!({{ \"{}\": search_repos_data }});\n",
                    target_field
                ));
            }
        }
        "search_issues" => {
            let q = get_str_param(node, ctx, "q", "rust");
            let sort = get_str_param(node, ctx, "sort", "comments");
            let order = get_str_param(node, ctx, "order", "desc");
            let per_page = get_i64_param(node, "per_page", 10);

            code.push_str(&format!(
                "{ind}// Search issues and pull requests query: \"{q}\"\n"
            ));
            code.push_str(&format!("{ind}let root_data = octocrab.search()\n"));
            code.push_str(&format!(
                "{ind}    .issues_and_pull_requests(\"{q}\")\n{ind}    .sort(\"{sort}\")\n{ind}    .order(\"{order}\")\n{ind}    .per_page({per_page})\n{ind}    .send()\n{ind}    .await?;\n"
            ));
            aggregators.push("root_data".to_string());
        }
        _ => {
            code.push_str(&format!(
                "{ind}// Entity {} (not yet fully implemented in codegen)\n",
                node.entity_id
            ));
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
