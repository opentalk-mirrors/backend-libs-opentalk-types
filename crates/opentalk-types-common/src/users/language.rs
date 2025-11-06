// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use icu_locid::{LanguageIdentifier, langid};

use crate::utils::ExampleData;

/// A language identifier
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
    derive_more::From,
    derive_more::Into,
    derive_more::AsRef,
    derive_more::AsMut,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde_with::DeserializeFromStr)
)]
pub struct Language(pub LanguageIdentifier);

#[cfg(feature = "bincode")]
mod bincode_impls {
    use bincode::{
        BorrowDecode, Decode, Encode,
        de::{BorrowDecoder, Decoder},
        enc::Encoder,
        error::{DecodeError, EncodeError},
    };

    use super::Language;

    impl Encode for Language {
        fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            Encode::encode(&self.0.to_string(), encoder)
        }
    }

    impl<Context> Decode<Context> for Language {
        fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
            let decoded: String = Decode::<Context>::decode(decoder)?;
            let language_identifier = decoded.parse().map_err(|e| {
                DecodeError::OtherString(format!("Invalid language identifier: {e}"))
            })?;
            Ok(Self(language_identifier))
        }
    }

    impl<'de, Context> BorrowDecode<'de, Context> for Language {
        fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
            decoder: &mut D,
        ) -> Result<Self, DecodeError> {
            let decoded: String = Decode::<Context>::decode(decoder)?;
            let language_identifier = decoded.parse().map_err(|e| {
                DecodeError::OtherString(format!("Invalid language identifier: {e}"))
            })?;
            Ok(Self(language_identifier))
        }
    }
}

impl Language {
    /// Create a new empty [`Language`]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(feature = "utoipa")]
mod impl_to_schema {
    //! The `#[derive(utoipa::ToSchema)] implementation does not yet properly support
    //! exposing schema information of types wrapped by the NewType pattern, therefore
    //! a manual implementation is required for now.
    //! Issue: <https://github.com/juhaku/utoipa/issues/663>

    use serde_json::json;
    use utoipa::{
        PartialSchema, ToSchema,
        openapi::{ObjectBuilder, RefOr, Schema, Type},
    };

    use super::Language;
    use crate::utils::ExampleData as _;

    impl PartialSchema for Language {
        fn schema() -> RefOr<Schema> {
            ObjectBuilder::new()
                .schema_type(Type::String)
                .description(Some("A language identifier"))
                .format(Some(utoipa::openapi::SchemaFormat::Custom(
                    "bcp-47".to_string(),
                )))
                .examples([json!(Language::example_data())])
                .into()
        }
    }

    impl ToSchema for Language {
        fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
            schemas.push((Self::name().into(), Self::schema()));
        }
    }
}

impl ExampleData for Language {
    fn example_data() -> Self {
        Self(langid!("de"))
    }
}

#[cfg(test)]
mod tests {
    use icu_locid::langid;
    use pretty_assertions::assert_eq;

    use super::Language;

    #[test]
    fn parse() {
        assert_eq!(
            "de-AT".parse::<Language>().unwrap(),
            Language(langid!("de-AT"))
        );
        assert_eq!("xx".parse::<Language>().unwrap(), Language(langid!("xx")));
    }

    #[test]
    fn parse_invalid() {
        assert!("".parse::<Language>().is_err());
        assert!("🚀".parse::<Language>().is_err());
    }
}
