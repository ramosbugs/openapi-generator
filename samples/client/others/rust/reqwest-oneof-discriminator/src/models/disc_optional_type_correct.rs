/*
 * fruity
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscOptionalTypeCorrect {
    #[serde(rename = "fruitType", skip_serializing_if = "Option::is_none")]
    pub fruit_type: Option<String>,
}

impl DiscOptionalTypeCorrect {
    pub fn new() -> DiscOptionalTypeCorrect {
        DiscOptionalTypeCorrect {
            fruit_type: None,
        }
    }
}


