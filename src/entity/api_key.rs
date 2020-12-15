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

#[derive(Clone, Debug, PartialEq, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;


    #[test]
    fn test_to_json_for_Err() {
        let dto: anyhow::Result<ApiKeyDto> = Err(anyhow::Error::msg("DTO error"));
        assert!(dto.to_json().is_err())
    }

    #[test]
    fn test_to_json_for_ApiKeyDto() {
        let dto: anyhow::Result<ApiKeyDto> = Ok(ApiKeyDto {
            key: "111-111-111".to_string(),
            application_id: "000-000-000".to_string(),
        });
        let json_control = r#"{"key":"111-111-111","application_id":"000-000-000"}"#.to_string();
        assert_eq!(dto.to_json().unwrap(), json_control);
    }

    #[test]
    fn test_dto_try_from_vec_ApiDto_single() {
        let vec_dto: Vec<ApiKeyDto> = vec![
            ApiKeyDto {key: "000-000".to_string(), application_id: "111-111".to_string()},
        ];
        let dto: ApiKeyDto = vec_dto.try_into().unwrap();
        assert_eq!(dto, ApiKeyDto {key: "000-000".to_string(), application_id: "111-111".to_string()});
    }

    #[test]
    fn test_dto_try_from_vec_ApiDto_multiple() {
        let vec_dto: Vec<ApiKeyDto> = vec![
            ApiKeyDto {key: "000-000".to_string(), application_id: "111-111".to_string()},
            ApiKeyDto {key: "222-222".to_string(), application_id: "333-333".to_string()},
        ];
        let dto: anyhow::Result<ApiKeyDto> = vec_dto.try_into();
        assert!(dto.is_err());
    }

    #[test]
    fn test_dto_from_bo() {
        let bo: ApiKeyBo = ApiKeyBo {key: "000-000".to_string(), applicationId: "\"111-111\"".to_string()};
        let dto: ApiKeyDto = ApiKeyDto {key: "000-000".to_string(), application_id: "111-111".to_string()};
        assert_eq!(ApiKeyDto::from(bo), dto);
    }
}