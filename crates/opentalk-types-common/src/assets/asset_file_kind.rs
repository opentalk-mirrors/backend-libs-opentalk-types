// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::str::FromStr;

use snafu::Snafu;

use crate::utils::ExampleData;

/// The minimum length for a file kind
pub const ASSET_FILE_KIND_MIN_LENGTH: usize = 1;
/// The maximum length for a file kind
pub const ASSET_FILE_KIND_MAX_LENGTH: usize = 20;

/// The kind of asset
///
/// Can be parsed using [`std::str::FromStr`].
/// May contain alphanumeric ascii characters and underscores only, and is restricted to a
/// length of [`ASSET_FILE_KIND_MIN_LENGTH`] up to [`ASSET_FILE_KIND_MAX_LENGTH`] characters.
/// This serves as sanitization measure for user input.
#[derive(Debug, Clone, Default, PartialEq, Eq, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde_with::DeserializeFromStr)
)]
pub struct AssetFileKind(String);

#[derive(Debug, Snafu)]
pub enum ParseAssetFileKindError {
    #[snafu(display("AssetFileKind must be at least {min_length} characters long"))]
    TooShort { min_length: usize },

    #[snafu(display("AssetFileKind must not be longer than {max_length} characters"))]
    TooLong { max_length: usize },

    #[snafu(display("AssetFileKind only allows alphanumeric ascii characters or '_'"))]
    InvalidCharacters,
}

impl FromStr for AssetFileKind {
    type Err = ParseAssetFileKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < ASSET_FILE_KIND_MIN_LENGTH {
            return Err(ParseAssetFileKindError::TooShort {
                min_length: ASSET_FILE_KIND_MIN_LENGTH,
            });
        }
        if s.len() > ASSET_FILE_KIND_MAX_LENGTH {
            return Err(ParseAssetFileKindError::TooLong {
                max_length: ASSET_FILE_KIND_MAX_LENGTH,
            });
        }
        if s.chars().any(|c| !(c.is_ascii_alphanumeric() || c == '_')) {
            return Err(ParseAssetFileKindError::InvalidCharacters);
        }
        Ok(AssetFileKind(s.into()))
    }
}

#[cfg(feature = "utoipa")]
mod impl_utoipa {
    use serde_json::json;
    use utoipa::{
        openapi::{ObjectBuilder, RefOr, Schema, Type},
        PartialSchema, ToSchema,
    };

    use super::{AssetFileKind, ASSET_FILE_KIND_MAX_LENGTH, ASSET_FILE_KIND_MIN_LENGTH};
    use crate::utils::ExampleData;

    impl PartialSchema for AssetFileKind {
        fn schema() -> RefOr<Schema> {
            ObjectBuilder::new()
                .schema_type(Type::String)
                .min_length(Some(ASSET_FILE_KIND_MIN_LENGTH))
                .max_length(Some(ASSET_FILE_KIND_MAX_LENGTH))
                .pattern(Some("^[0-9a-zA-Z_]*$".to_string()))
                .description(Some("An asset file kind"))
                .examples([json!(AssetFileKind::example_data())])
                .into()
        }
    }

    impl ToSchema for AssetFileKind {
        fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
            schemas.push((Self::name().into(), Self::schema()));
        }
    }
}

impl ExampleData for AssetFileKind {
    fn example_data() -> Self {
        Self("mykind".to_string())
    }
}
