mod models;
mod api;
mod page;

pub mod pulls {
    pub use crate::api::pulls::PullRequestState;
}

use serde::Serialize;

type Result<T, E=Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub struct Octocrab {
    auth: Auth,
    client: reqwest::Client,
}

impl Octocrab {
    pub fn pulls(&self, owner: impl Into<String>, repo: impl Into<String>) -> api::pulls::PullRequestHandler {
        api::pulls::PullRequestHandler::new(
            self,
            owner.into(),
            repo.into(),
        )
    }

    pub async fn get<P: Serialize + ?Sized, R: FromResponse>(&self, url: String, parameters: Option<&P>) -> Result<R> {
        let full_url = format!("https://api.github.com{url}", url = url);
        let mut request = self.client.get(&full_url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        let response = self.send_request(request).await?;

        R::from_response(response).await
    }

    async fn send_request(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        Ok(request.send().await?)
    }
}

#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> Result<Self> {
        Ok(response.json().await?)
    }
}

impl Default for Octocrab {
    fn default() -> Self {
        Self {
            auth: Auth::default(),
            client: reqwest::ClientBuilder::new().user_agent("octocrab").build().unwrap()
        }
    }
}

pub enum Auth {
    None,
    PersonalToken(String),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}
