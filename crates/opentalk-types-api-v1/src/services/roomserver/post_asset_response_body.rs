// SPDX-License-Identifier: EUPL-1.2
// SPDX-FileCopyrightText: OpenTalk Team <mail@opentalk.eu>

use crate::assets::AssetResource;

/// Response for the `POST /services/roomserver/room/{room_id}/asset` endpoint
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostAssetResponseBody {
    /// The asset that was uploaded
    pub asset_resource: AssetResource,

    /// The remaining quota in bytes for the room, if applicable.
    pub remaining_quota_bytes: Option<u64>,
}
