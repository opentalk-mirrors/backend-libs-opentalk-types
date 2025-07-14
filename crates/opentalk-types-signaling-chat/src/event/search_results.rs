// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use crate::{state::ChatChunk, Scope};

/// Results from a search in the chat history
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SearchResults {
    /// A chunk of messages matching the search term
    pub matches: ChatChunk,
    /// The [`Scope`] of the messages
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub scope: Scope,
}
