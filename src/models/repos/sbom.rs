use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbomGenerateReportResponse {
    pub sbom_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SbomCreationInfo {
    pub created: DateTime<Utc>,
    pub creators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SbomPackageExternalRef {
    #[serde(rename = "referenceCategory")]
    pub reference_category: String,
    #[serde(rename = "referenceLocator")]
    pub reference_locator: String,
    #[serde(rename = "referenceType")]
    pub reference_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SbomPackage {
    #[serde(rename = "SPDXID")]
    pub spdxid: String,
    pub name: String,
    #[serde(rename = "versionInfo")]
    pub version_info: String,
    #[serde(rename = "downloadLocation")]
    pub download_location: String,
    #[serde(rename = "filesAnalyzed")]
    pub files_analyzed: bool,
    #[serde(rename = "licenseConcluded", skip_serializing_if = "Option::is_none")]
    pub license_concluded: Option<String>,
    #[serde(rename = "licenseDeclared", skip_serializing_if = "Option::is_none")]
    pub license_declared: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<String>,
    #[serde(rename = "copyrightText", skip_serializing_if = "Option::is_none")]
    pub copyright_text: Option<String>,
    #[serde(rename = "externalRefs", skip_serializing_if = "Option::is_none")]
    pub external_refs: Option<Vec<SbomPackageExternalRef>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SbomRelationship {
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    #[serde(rename = "spdxElementId")]
    pub spdx_element_id: String,
    #[serde(rename = "relatedSpdxElement")]
    pub related_spdx_element: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SbomDependencyGraph {
    #[serde(rename = "SPDXID")]
    pub spdxid: String,
    #[serde(rename = "spdxVersion")]
    pub spdx_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "creationInfo")]
    pub creation_info: SbomCreationInfo,
    pub name: String,
    #[serde(rename = "dataLicense")]
    pub data_license: String,
    #[serde(rename = "documentNamespace")]
    pub document_namespace: String,
    pub packages: Vec<SbomPackage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<SbomRelationship>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SbomFetchResponse {
    NotFound,
    NotReady,
    Ready { graph: Box<SbomDependencyGraph> },
    Broken,
}
