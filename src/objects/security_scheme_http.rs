use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiSecuritySchemeHttp<SecuritySchemeHttpExt> {
    description: Option<String>,
    scheme: String,
    bearer_format: Option<String>,
    #[serde(flatten)]
    extension: SecuritySchemeHttpExt,
}
