// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Represents a list of invalid fields in a request or data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InvalidFields {
    /// A list of field names that are invalid.
    pub fields: Vec<String>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization() {
        let produced = serde_json::to_value(InvalidFields {
            fields: vec!["Test Field".to_string()],
        })
        .unwrap();

        let expected = json!({ "fields": ["Test Field"] });

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization() {
        let produced: InvalidFields =
            serde_json::from_value(json!({ "fields": ["Test Field"] })).unwrap();

        let expected = InvalidFields {
            fields: vec!["Test Field".to_string()],
        };

        assert_eq!(produced, expected);
    }
}
