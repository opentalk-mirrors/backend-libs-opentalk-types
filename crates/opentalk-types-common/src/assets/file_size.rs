// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use snafu::{Snafu, ensure};

use crate::utils::ExampleData;

const DEFAULT_VALUE: i64 = 0;
const ZERO_VALUE: i64 = 0;
const MIN_VALUE: i64 = 0;
const MAX_VALUE: i64 = i64::MAX;

/// Error when parsing an [`FileSize`].
#[derive(Debug, Snafu)]
pub enum TryFromFileSizeError {
    /// File size is larger than the maximum file size allowed.
    #[snafu(display("File size is larger than the maximum file size allowed ({file_size_max})."))]
    ValueTooLarge {
        /// The maximum file size
        file_size_max: i64,
    },
    /// File size is negative.
    #[snafu(display("File size is negative"))]
    ValueNegative,
}

/// The size of a file.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::AsRef,
    derive_more::Into,
)]
#[cfg_attr(
    feature = "diesel",
    derive(
        opentalk_diesel_newtype::DieselNewtype,
        diesel::expression::AsExpression,
        diesel::deserialize::FromSqlRow
    )
)]
#[cfg_attr(feature="diesel", diesel(sql_type = diesel::sql_types::BigInt))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "i64")
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema), schema(example = json!(FileSize::example_data())))]
pub struct FileSize(i64);

impl FileSize {
    /// The minimum value of the type.
    pub const MIN: Self = Self(MIN_VALUE);

    /// The maximum value of the type.
    pub const MAX: Self = Self(MAX_VALUE);

    /// The default value of the type.
    pub const DEFAULT: Self = Self(DEFAULT_VALUE);

    /// The zero value of the type.
    pub const ZERO: Self = Self(ZERO_VALUE);

    /// Add two [`FileSize`] values, saturating.
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Subtract an [`FileSize`] from another, saturating.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl TryFrom<u64> for FileSize {
    type Error = TryFromFileSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromFileSizeError::ValueTooLarge {
            file_size_max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<usize> for FileSize {
    type Error = TryFromFileSizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_e| TryFromFileSizeError::ValueTooLarge {
            file_size_max: MAX_VALUE,
        })?;
        Self::try_from(value)
    }
}

impl TryFrom<i64> for FileSize {
    type Error = TryFromFileSizeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        ensure!(value >= 0, ValueNegativeSnafu);
        Ok(Self(value))
    }
}

impl From<u32> for FileSize {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<FileSize> for usize {
    fn from(value: FileSize) -> Self {
        value.0 as usize
    }
}

impl ExampleData for FileSize {
    fn example_data() -> Self {
        Self(17)
    }
}
