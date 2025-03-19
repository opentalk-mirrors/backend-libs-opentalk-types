// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// Moderator command, select the speaker
#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", tag = "how")
)]
pub enum Select {
    /// Revoke speaker status if exists, do no select a new one
    None,

    /// Select a random speaker
    Random,

    /// Advance the moderation depending on the selection strategy.
    /// Can just unset the current speaker if selection strategy is nomination
    Next,

    /// Select a specific participant
    Specific(SelectSpecific),
}

/// Fields that are provided when issuing the [`Select::Specific`] command
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SelectSpecific {
    /// The participant to be selected
    pub participant: ParticipantId,

    /// If true the selected participant will not be removed from either the allow- or playlist
    pub keep_in_remaining: bool,
}
