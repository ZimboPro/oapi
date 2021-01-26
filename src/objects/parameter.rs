use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Getters, Sparsable, OApiCheck)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct OApiParameter<
    ParameterExt,
    ExampleExt,
    ObjectExt,
    ArrayExt,
    NumericExt,
    StringExt,
    DiscriminatorExt,
    ExternalDocExt,
> {
    name: String,
    #[serde(rename = "in")]
    in_: OApiParameterLocation,
    description: Option<String>,
    required: Option<bool>,
    #[serde(default)]
    deprecated: bool,
    #[serde(default)]
    allow_empty_value: bool,
    style: Option<OApiParameterStyle>,
    #[serde(default)]
    explode: bool,
    #[serde(default)]
    allow_reserved: bool,
    schema: Option<
        OperatorSelector<
            OApiSchema<
                ObjectExt,
                ArrayExt,
                NumericExt,
                StringExt,
                DiscriminatorExt,
                ExternalDocExt,
            >,
        >,
    >,
    #[serde(flatten)]
    example: Option<OApiExampleSelector<ExampleExt>>,
    #[serde(flatten)]
    extension: ParameterExt,
}
