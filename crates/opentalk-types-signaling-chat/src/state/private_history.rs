// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

use crate::state::ChatChunk;

/// Private chat history
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrivateHistory {
    /// Private chat correspondent
    pub correspondent: ParticipantId,

    /// Private chat history chunk
    pub history: ChatChunk,
}
