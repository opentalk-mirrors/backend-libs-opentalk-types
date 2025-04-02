// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::borrow::Cow;

/// An entry in a validation error list
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ValidationErrorEntry {
    /// The field related to the error
    ///
    /// If the value is [`None`] that means the error happened at struct level
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub field: Option<Cow<'static, str>>,

    /// Machine readable error message
    pub code: Cow<'static, str>,

    /// Human readable error message
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub message: Option<Cow<'static, str>>,
}

impl ValidationErrorEntry {
    /// Create a new validation error entry for a field with an optional message
    pub fn new<F, C, M>(field: F, code: C, message: Option<M>) -> Self
    where
        F: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>,
        M: Into<Cow<'static, str>>,
    {
        Self {
            field: Some(field.into()),
            code: code.into(),
            message: message.map(Into::into),
        }
    }
}

#[cfg(feature = "bincode")]
impl<C> bincode::Decode<C> for ValidationErrorEntry {
    fn decode<D: bincode::de::Decoder<Context = C>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self {
            field: bincode::Decode::decode(decoder)?,
            code: bincode::Decode::decode(decoder)?,
            message: bincode::Decode::decode(decoder)?,
        })
    }
}

#[cfg(feature = "bincode")]
impl<C> bincode::BorrowDecode<'static, C> for ValidationErrorEntry {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'static, Context = C>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self {
            field: bincode::BorrowDecode::borrow_decode(decoder)?,
            code: bincode::BorrowDecode::borrow_decode(decoder)?,
            message: bincode::BorrowDecode::borrow_decode(decoder)?,
        })
    }
}
