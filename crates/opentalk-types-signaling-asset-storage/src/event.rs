// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling events for the `asset_storage` namespace

use opentalk_types_common::assets::FileSize;

/// Events sent out by the `asset_storage` module
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "message", rename_all = "snake_case")
)]
pub enum AssetStorageEvent {
    /// The used storage was updated.
    StorageUsageUpdate {
        /// The amount of used asset storage in bytes.
        used_storage: FileSize,
    },
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn storage_update_event() {
        let json = json!({
            "message": "storage_usage_update",
            "used_storage": 123,
        });

        let msg: AssetStorageEvent = serde_json::from_value(json).unwrap();

        assert_eq!(
            msg,
            AssetStorageEvent::StorageUsageUpdate {
                used_storage: FileSize::from(123u32),
            }
        );
    }
}
