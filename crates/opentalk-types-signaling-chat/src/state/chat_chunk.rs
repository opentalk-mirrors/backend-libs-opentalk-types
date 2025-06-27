// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Chat history is retrieved in chunks. This is to avoid sending a giant list
//! of chat messages when a participant joins.

use crate::state::StoredMessage;

/// The maximum number of messages that a [`ChatChunk`] can contain
pub const CHAT_CHUNK_SIZE: u64 = 100;

/// A chunk of the chat message history
///
/// The specific messages in a chunk will depend on the clients request. The first
/// [`ChatChunk`] is received when joining the room, containing the most recent
/// messages and the index to the next chunk. Messages are ordered chronologically,
/// the next chunk always contains older messages than the current one.
///
/// Further requests to fetch message history chunks need to contain the index
/// received with the previous chunk. A missing index indicates that no older
/// messages exist.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatChunk {
    /// The messages in this chunk
    pub messages: Vec<StoredMessage>,
    /// The message index of the newest message of the next chunk. Must be provided
    /// when requesting the next chunk.
    pub next_index: Option<u64>,
}
