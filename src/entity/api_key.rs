#![allow(non_snake_case)]

use std::convert::TryFrom;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use serde::Serialize;

use crate::helpers::string_helper::StringExtensions;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct ApiKeyBo {
    key: String,
    applicationId: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyDto {
    pub key: String,
    pub application_id: String,
}

impl From<ApiKeyBo> for ApiKeyDto {
    fn from(raw_struct: ApiKeyBo) -> Self {
        ApiKeyDto {
            key: raw_struct.key,
            application_id: raw_struct.applicationId.remove_quotes(),
        }
    }
}

impl TryFrom<Vec<ApiKeyDto>> for ApiKeyDto {
    type Error = anyhow::Error;

    fn try_from(single_value_vec: Vec<ApiKeyDto>) -> anyhow::Result<Self> {
        if single_value_vec.len() == 1 {
            Ok(single_value_vec.into_iter().next().unwrap())
        } else {
            anyhow::Result::Err(anyhow::Error::msg("More than one result for given API Key"))
        }
    }
}

pub trait Jsonify {
    fn to_json(self) -> anyhow::Result<String>;
}

impl<T: Serialize> Jsonify for anyhow::Result<T> {
    fn to_json(self) -> anyhow::Result<String> {
        self
            .and_then(|it| serde_json::to_string(&it).map_err(anyhow::Error::msg))
            .map_err(anyhow::Error::msg)
    }
}