#![allow(non_snake_case)]
use crate::helpers::string_helper::StringExtensions;

use std::convert::TryFrom;
use serde::Serialize;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use anyhow::{Result, Error};

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
    type Error = &'static str;

    fn try_from(single_value_vec: Vec<ApiKeyDto>) -> std::result::Result<Self, &'static str> {
        if single_value_vec.len() == 1 {
            Ok(single_value_vec.into_iter().next().unwrap())
        } else {
            Err("More than one result for given API Key")
        }
    }
}

pub trait Jsonify {
    fn to_json(&self) -> Result<String>;
}

impl Jsonify for Option<Vec<ApiKeyDto>> {
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(|e| Error::from(e).context("Error while converting to Json"))
    }
}

impl Jsonify for Option<ApiKeyDto> {
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(|e| Error::from(e).context("Error while converting to Json"))
    }
}