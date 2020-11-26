#![allow(non_snake_case)]
use super::super::helpers::string_helper::remove_quotes;

use std::convert::TryFrom;

use serde::Serialize;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize)]
pub struct ApiKeyRawStruct {
    key: String,
    applicationId: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiKeyStruct {
    pub key: String,
    pub application_id: String,
}

impl From<ApiKeyRawStruct> for ApiKeyStruct {
    fn from(raw_struct: ApiKeyRawStruct) -> Self {
        ApiKeyStruct {
            key: raw_struct.key,
            application_id: remove_quotes(raw_struct.applicationId),
        }
    }
}

impl TryFrom<Vec<ApiKeyStruct>> for ApiKeyStruct {
    type Error = &'static str;

    fn try_from(single_value_vec: Vec<ApiKeyStruct>) -> std::result::Result<Self, &'static str> {
        if single_value_vec.len() == 1 {
            Ok(single_value_vec.into_iter().nth(0).unwrap())
        } else {
            Err("More than one result for given API Key")
        }
    }
}