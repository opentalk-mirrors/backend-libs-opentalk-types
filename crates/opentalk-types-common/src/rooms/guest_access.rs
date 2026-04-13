// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{sql_enum, utils::ExampleData};

sql_enum!(
    feature_gated:

    #[derive( Eq, PartialEq)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(rename_all = "snake_case")
    )]
    #[cfg_attr(
        feature = "utoipa",
        derive(utoipa::ToSchema),
        schema(example = json!(GuestAccess::example_data()))
    )]
    GuestAccess,
    "guest_access",
    GuestAccessType,
    {
        Disabled = b"disabled",
        WaitingRoom = b"waiting_room",
        DirectAccess = b"direct_access",
    }
);

#[allow(clippy::derivable_impls)]
impl Default for GuestAccess {
    fn default() -> Self {
        Self::DirectAccess
    }
}

impl ExampleData for GuestAccess {
    fn example_data() -> Self {
        Self::DirectAccess
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::GuestAccess;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(GuestAccess::Disabled).unwrap(),
            json!("disabled")
        );
        assert_eq!(
            serde_json::to_value(GuestAccess::WaitingRoom).unwrap(),
            json!("waiting_room")
        );
        assert_eq!(
            serde_json::to_value(GuestAccess::DirectAccess).unwrap(),
            json!("direct_access")
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::from_value::<GuestAccess>(json!("disabled")).unwrap(),
            GuestAccess::Disabled
        );
        assert_eq!(
            serde_json::from_value::<GuestAccess>(json!("waiting_room")).unwrap(),
            GuestAccess::WaitingRoom
        );
        assert_eq!(
            serde_json::from_value::<GuestAccess>(json!("direct_access")).unwrap(),
            GuestAccess::DirectAccess
        );
    }
}
