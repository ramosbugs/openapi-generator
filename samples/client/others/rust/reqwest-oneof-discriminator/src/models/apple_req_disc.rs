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
pub struct AppleReqDisc {
    #[serde(rename = "seeds")]
    pub seeds: i32,
    #[serde(rename = "fruitType")]
    pub fruit_type: String,
}

impl AppleReqDisc {
    pub fn new(seeds: i32, fruit_type: String) -> AppleReqDisc {
        AppleReqDisc {
            seeds,
            fruit_type,
        }
    }
}


