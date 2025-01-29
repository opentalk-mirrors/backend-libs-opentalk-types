// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Types related to the API endpoints under `/rooms/{room_id}/assets`.

mod post_asset_query;
mod post_asset_response_body;
mod rooms_by_room_id_assets_get_response_body;

pub use post_asset_query::PostAssetQuery;
pub use post_asset_response_body::PostAssetResponseBody;
pub use rooms_by_room_id_assets_get_response_body::RoomsByRoomIdAssetsGetResponseBody;
