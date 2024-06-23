/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TotalErc1155 {
    #[serde(rename = "token_id")]
    pub token_id: String,
    #[serde(rename = "decimals")]
    pub decimals: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl TotalErc1155 {
    pub fn new(token_id: String, decimals: String, value: String) -> TotalErc1155 {
        TotalErc1155 {
            token_id,
            decimals,
            value,
        }
    }
}
