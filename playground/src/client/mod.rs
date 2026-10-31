use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[derive(Clone, Debug, Default)]
pub struct ApiResponse {
    pub value: serde_json::Value,
    pub rate_limit_remaining: Option<u64>,
    pub rate_limit_limit: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct GithubClient {
    token: Option<String>,
    last_remaining: Arc<AtomicI64>,
    last_limit: Arc<AtomicI64>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Self {
        let token = token.and_then(|t| {
            let trimmed = t.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });
        Self {
            token,
            last_remaining: Arc::new(AtomicI64::new(-1)),
            last_limit: Arc::new(AtomicI64::new(-1)),
        }
    }

    pub fn rate_limit_remaining(&self) -> Option<u64> {
        let val = self.last_remaining.load(Ordering::SeqCst);
        if val >= 0 { Some(val as u64) } else { None }
    }

    pub fn rate_limit_limit(&self) -> Option<u64> {
        let val = self.last_limit.load(Ordering::SeqCst);
        if val >= 0 { Some(val as u64) } else { None }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn fetch(
        &self,
        route: &str,
        params: &HashMap<String, String>,
    ) -> Result<ApiResponse, String> {
        let crab = match &self.token {
            Some(token) => octocrab::Octocrab::builder()
                .personal_token(token.clone())
                .build()
                .map_err(|e| format!("Failed to create Octocrab instance: {e}"))?,
            None => (*octocrab::instance()).clone(),
        };

        let response: Result<serde_json::Value, octocrab::Error> = if params.is_empty() {
            crab.get(route, None::<&()>).await
        } else {
            crab.get(route, Some(params)).await
        };

        match response {
            Ok(value) => {
                let mut rate_limit_remaining = None;
                let mut rate_limit_limit = None;

                // Also attempt to read rate limit if available
                if let Ok(rate) = crab.ratelimit().get().await {
                    let rem = rate.rate.remaining as u64;
                    let lim = rate.rate.limit as u64;
                    self.last_remaining.store(rem as i64, Ordering::SeqCst);
                    self.last_limit.store(lim as i64, Ordering::SeqCst);
                    rate_limit_remaining = Some(rem);
                    rate_limit_limit = Some(lim);
                }

                Ok(ApiResponse {
                    value,
                    rate_limit_remaining,
                    rate_limit_limit,
                })
            }
            Err(e) => Err(format!("GitHub API error for {route}: {e}")),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn fetch(
        &self,
        route: &str,
        params: &HashMap<String, String>,
    ) -> Result<ApiResponse, String> {
        let _start = Instant::now();
        let client = reqwest::Client::new();

        let clean_route = route.strip_prefix('/').unwrap_or(route);
        let mut url = format!("https://api.github.com/{clean_route}");

        if !params.is_empty() {
            let query_str = serde_urlencoded::to_string(params)
                .map_err(|e| format!("Failed to encode params: {e}"))?;
            url = format!("{url}?{query_str}");
        }

        let mut request = client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {token}"));
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Network request failed: {e}"))?;

        let headers = response.headers().clone();
        let rate_limit_remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());
        let rate_limit_limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        if let Some(rem) = rate_limit_remaining {
            self.last_remaining.store(rem as i64, Ordering::SeqCst);
        }
        if let Some(lim) = rate_limit_limit {
            self.last_limit.store(lim as i64, Ordering::SeqCst);
        }

        let status = response.status();
        let body_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {e}"))?;

        let value: serde_json::Value = serde_json::from_str(&body_text)
            .unwrap_or_else(|_| serde_json::Value::String(body_text.clone()));

        if !status.is_success() {
            let msg = value
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or(&body_text);
            return Err(format!("GitHub API error ({status}): {msg}"));
        }

        Ok(ApiResponse {
            value,
            rate_limit_remaining,
            rate_limit_limit,
        })
    }
}
