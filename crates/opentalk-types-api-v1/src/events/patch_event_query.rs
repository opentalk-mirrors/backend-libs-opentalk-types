// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::pagination::ItemCount;

/// Path query parameters for the `PATCH /events/{event_id}` endpoint
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
pub struct PatchEventQuery {
    /// Maximum number of invitees to include inside the event
    #[cfg_attr(feature = "serde", serde(default))]
    pub invitees_max: ItemCount,

    /// Flag to disable email notification
    #[cfg_attr(feature = "serde", serde(default))]
    pub suppress_email_notification: bool,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::PatchEventQuery;

    #[test]
    fn serialize_default() {
        let example = PatchEventQuery::default();
        assert_eq!(
            json!(example),
            json!({"invitees_max": 0, "suppress_email_notification": false})
        );
    }

    #[test]
    fn serialize() {
        let example = PatchEventQuery {
            invitees_max: 50.into(),
            suppress_email_notification: true,
        };
        assert_eq!(
            json!(example),
            json!({"invitees_max": 50, "suppress_email_notification": true})
        );
    }

    #[test]
    fn deserialize_default() {
        let example = PatchEventQuery::default();
        assert_eq!(example, serde_json::from_value(json!({})).unwrap());
    }

    #[test]
    fn deserialize() {
        let example = PatchEventQuery {
            invitees_max: 65.into(),
            suppress_email_notification: true,
        };
        assert_eq!(
            example,
            serde_json::from_value(
                json!({"invitees_max": 65, "suppress_email_notification": true})
            )
            .unwrap()
        );
    }
}
