// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// A modification of the remaining list has taken place, because someone edited the list by hand or
/// it got modified because a participant left/joined
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemainingUpdated {
    /// List of remaining participants in selection pool
    pub remaining: Vec<ParticipantId>,
}
