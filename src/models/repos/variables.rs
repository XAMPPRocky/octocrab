use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryVariable {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositoryVariables {
    pub total_count: i32,
    pub variables: Vec<RepositoryVariable>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateRepositoryVariable<'a> {
    /// Value for your secret,
    /// encrypted with LibSodium using the public key retrieved from the Get an organization public key endpoint.
    pub name: &'a str,
    /// ID of the key you used to encrypt the secret.
    pub value: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateRepositoryVariableResponse {
    Created,
    Updated,
}
