// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: OpenTalk Team <mail@opentalk.eu>

//! Signaling state for the `asset_storage` namespace

use opentalk_types_common::assets::FileSize;

/// State of the asset storage module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetStorageState {
    /// The amount of used asset storage in bytes.
    pub used_storage: FileSize,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModuleFrontendData for AssetStorageState {
    const NAMESPACE: Option<opentalk_types_common::modules::ModuleId> = Some(crate::MODULE_ID);
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn with_quota() {
        let json = json!({
            "used_storage": 123,
        });

        let msg: AssetStorageState = serde_json::from_value(json).unwrap();

        assert_eq!(
            msg,
            AssetStorageState {
                used_storage: FileSize::from(123u32),
            }
        );
    }
}
