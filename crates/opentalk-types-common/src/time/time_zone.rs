// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use derive_more::{AsRef, Deref, Display, From, FromStr, Into};

use crate::utils::ExampleData;

/// Representation of a timezone
#[derive(AsRef, Deref, Display, From, FromStr, Into, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature = "diesel",  diesel(sql_type = diesel::sql_types::Text))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeZone(chrono_tz::Tz);

impl TimeZone {
    /// Create a new UTC [`TimeZone`]
    pub fn utc() -> Self {
        Self(chrono_tz::Tz::Etc__UTC)
    }
}

impl Default for TimeZone {
    /// Create a default [`TimeZone`] (which is UTC).
    fn default() -> Self {
        TimeZone::utc()
    }
}

impl chrono::TimeZone for TimeZone {
    type Offset = chrono_tz::TzOffset;

    fn from_offset(offset: &Self::Offset) -> Self {
        chrono_tz::Tz::from_offset(offset).into()
    }

    fn offset_from_local_date(
        &self,
        local: &chrono::NaiveDate,
    ) -> chrono::MappedLocalTime<Self::Offset> {
        self.0.offset_from_local_date(local)
    }

    fn offset_from_local_datetime(
        &self,
        local: &chrono::NaiveDateTime,
    ) -> chrono::MappedLocalTime<Self::Offset> {
        self.0.offset_from_local_datetime(local)
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> Self::Offset {
        self.0.offset_from_utc_date(utc)
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> Self::Offset {
        self.0.offset_from_utc_datetime(utc)
    }
}

#[cfg(feature = "diesel")]
mod diesel_traits {
    use std::{
        io::Write,
        str::{FromStr, from_utf8},
    };

    use chrono_tz::Tz;
    use diesel::{
        backend::Backend,
        deserialize::{self, FromSql},
        pg::Pg,
        serialize::{self, IsNull, Output, ToSql},
    };

    use super::*;

    impl ToSql<diesel::sql_types::Text, Pg> for TimeZone {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
            write!(out, "{}", self.0)?;
            Ok(IsNull::No)
        }
    }

    impl FromSql<diesel::sql_types::Text, Pg> for TimeZone {
        fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            let s = from_utf8(bytes.as_bytes())?;
            let tz = Tz::from_str(s)?;

            Ok(Self(tz))
        }
    }
}

impl ExampleData for TimeZone {
    fn example_data() -> Self {
        chrono_tz::Europe::Berlin.into()
    }
}

#[cfg(feature = "utoipa")]
mod impl_utoipa {
    use serde_json::json;
    use utoipa::{
        PartialSchema, ToSchema,
        openapi::{ObjectBuilder, RefOr, Schema, Type},
    };

    use super::TimeZone;
    use crate::utils::ExampleData as _;

    impl PartialSchema for TimeZone {
        fn schema() -> RefOr<Schema> {
            ObjectBuilder::new()
                .schema_type(Type::String)
                .description(Some("A time zone"))
                .examples([json!(Self::example_data())])
                .into()
        }
    }

    impl ToSchema for TimeZone {
        fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
            schemas.push((Self::name().into(), Self::schema()));
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::TimeZone;

    #[test]
    fn test_reference() {
        let timezone = TimeZone::default();
        example_tz(&timezone);

        fn example_tz(_tz: &chrono_tz::Tz) {}
    }

    #[test]
    fn test_usage_as_chrono_timezone() {
        let dt = DateTime::from_timestamp(100000, 0).unwrap();
        let _with_timezone = dt.with_timezone(&TimeZone::default());
    }
}
