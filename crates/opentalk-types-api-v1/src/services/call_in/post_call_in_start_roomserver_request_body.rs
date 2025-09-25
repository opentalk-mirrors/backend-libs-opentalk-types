// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::{
    call_in::{CallInId, CallInPassword},
    users::DisplayName,
};

/// Body for the `POST /services/call_in/start` endpoint
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostCallInStartRoomServerRequestBody {
    /// The call-in ID
    pub id: CallInId,
    /// The call-in password
    pub pin: CallInPassword,
    /// The caller's display name, as defined by the call-in service.
    pub display_name: DisplayName,
}
