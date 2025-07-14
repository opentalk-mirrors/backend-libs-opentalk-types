// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::Scope;

/// Search in the chat history
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SearchHistory {
    /// The scope to search in
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub scope: Scope,
    /// The search term
    pub term: String,
    /// The message index of the [`ChatChunk`](crate::state::ChatChunk) in the
    /// search history. Retrieves the latest [`ChatChunk`](crate::state::ChatChunk)
    /// when [`None`].
    pub message_index: Option<u64>,
}
