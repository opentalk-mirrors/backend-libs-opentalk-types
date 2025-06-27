// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::Scope;

/// Gets a chunk of the message history in the specified `scope`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetHistoryChunk {
    /// Determines which [`ChatChunk`](crate::state::ChatChunk) is requested.
    /// This is always the newest message of the chunk.
    pub message_index: u64,

    /// The scope of the chat history
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub scope: Scope,
}
