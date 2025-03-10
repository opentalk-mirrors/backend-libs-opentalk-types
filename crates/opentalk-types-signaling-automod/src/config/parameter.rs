// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use crate::config::SelectionStrategy;

/// Parameter that are used to describe the configuration of the automod session
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Parameter {
    /// The strategy used to determine the next speaker
    pub selection_strategy: SelectionStrategy,

    /// Is `list` visible to the frontend
    pub show_list: bool,

    /// If a raised hand should add a participant into `list`
    pub consider_hand_raise: bool,

    /// Time limit each speaker has before its speaking status get revoked
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            with = "duration_millis",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub time_limit: Option<Duration>,

    /// Depending on the `selection_strategy` this will prevent participants to become
    /// speaker twice in a single automod session
    pub allow_double_selection: bool,

    /// The frontend will play an animation when a random selection
    /// is being made
    pub animation_on_random: bool,

    /// Append the allow_list or playlist with joining participants, depending on the `selection_strategy`
    pub auto_append_on_join: bool,
}

#[cfg(feature = "serde")]
mod duration_millis {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<u64>::deserialize(deserializer)?.map(Duration::from_millis))
    }

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(duration) = duration {
            serializer.serialize_u64(duration.as_millis() as u64)
        } else {
            serializer.serialize_none()
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use std::time::Duration;

    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;
    use crate::config::SelectionStrategy;

    #[test]
    fn parameter_options() {
        let produced = serde_json::to_value(Parameter {
            selection_strategy: SelectionStrategy::None,
            allow_double_selection: true,
            animation_on_random: true,
            auto_append_on_join: true,
            consider_hand_raise: true,
            show_list: true,
            time_limit: Some(Duration::from_millis(0)),
        })
        .unwrap();

        let expected = json!({
            "selection_strategy": "none",
            "show_list": true,
            "consider_hand_raise": true,
            "allow_double_selection": true,
            "animation_on_random": true,
            "auto_append_on_join": true,
            "time_limit": 0,
        });

        assert_eq!(produced, expected);

        let produced = serde_json::to_value(Parameter {
            selection_strategy: SelectionStrategy::None,
            allow_double_selection: false,
            animation_on_random: false,
            auto_append_on_join: false,
            consider_hand_raise: false,
            show_list: false,
            time_limit: None,
        })
        .unwrap();

        let expected = json!({
            "selection_strategy": "none",
            "show_list": false,
            "consider_hand_raise": false,
            "allow_double_selection": false,
            "animation_on_random": false,
            "auto_append_on_join": false,
        });

        assert_eq!(produced, expected);
    }

    #[derive(Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct TestStruct {
        #[cfg_attr(
            feature = "serde",
            serde(with = "duration_millis", skip_serializing_if = "Option::is_none")
        )]
        duration: Option<Duration>,
    }

    #[test]
    fn serialize_duration_millis_some() {
        let produced = serde_json::to_value(TestStruct {
            duration: Some(Duration::from_millis(12345)),
        })
        .unwrap();

        let expected = json!({"duration": 12345});

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialize_duration_millis_none() {
        let produced = serde_json::to_value(TestStruct { duration: None }).unwrap();

        let unexpected = json!({"duration": null});
        let expected = json!({});

        assert_ne!(produced, unexpected);
        assert_eq!(produced, expected);
    }

    #[test]
    #[should_panic]
    fn deserialize_duration_millis_overflow() {
        let _ =
            serde_json::from_value::<TestStruct>(json!({"duration": 18446744073709551616_u128}))
                .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_duration_millis_underflow() {
        let _ = serde_json::from_value::<TestStruct>(json!({"duration": -1})).unwrap();
    }
}
