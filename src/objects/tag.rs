use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiTag {
    name: String,
    description: Option<String>,
    external_docs: Option<OApiExternalDocumentation>,
    #[serde(flatten)]
    extension: HashMap<String, Value>,
}
