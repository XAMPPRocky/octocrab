use secrecy::{ExposeSecret, SecretString};

use crate::Octocrab;

#[derive(serde::Serialize)]
pub struct ExchangeWebFlowCodeBuilder<'octo, 'client_id, 'code, 'client_secret> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    client_id: &'client_id str,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<&'code str>,
    client_secret: &'client_secret str,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,
}

impl<'octo, 'client_id, 'code, 'client_secret>
    ExchangeWebFlowCodeBuilder<'octo, 'client_id, 'code, 'client_secret>
{
    pub(crate) fn new(
        crab: &'octo Octocrab,
        client_id: &'client_id SecretString,
        code: Option<&'code str>,
        client_secret: &'client_secret SecretString,
        redirect_uri: Option<String>,
    ) -> Self {
        Self {
            crab,
            client_id: client_id.expose_secret(),
            code,
            client_secret: client_secret.expose_secret(),
            redirect_uri,
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::Release> {
        let route = "/login/oauth/access_token";
        self.crab.post(route, Some(&self)).await
    }
}
