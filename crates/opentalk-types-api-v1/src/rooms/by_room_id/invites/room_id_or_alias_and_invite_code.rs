// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::rooms::{RoomIdOrAlias, invite_codes::InviteCode};

/// Path for *GET /rooms/{room_id_or_alias}/invites/{invite_code}*
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
pub struct RoomIdOrAliasAndInviteCode {
    /// The room id or alias for the invite
    pub room_id_or_alias: RoomIdOrAlias,

    /// The invite code id
    pub invite_code: InviteCode,
}
