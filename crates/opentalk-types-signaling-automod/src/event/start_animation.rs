// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// Tells the frontend to start a 'random' draw animation (e.g. wheel of names)
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StartAnimation {
    /// List of participant in the selction pool
    pub pool: Vec<ParticipantId>,

    /// Random participant that was selected as a speaker
    pub result: ParticipantId,
}
