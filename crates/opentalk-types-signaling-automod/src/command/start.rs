// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

use crate::config::Parameter;

/// Fields that are provided when issuing the start message
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Start {
    /// The parameters for the automod session
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parameter: Parameter,

    /// Depending on the selection strategy, the list of Participant that can be chosen from.
    ///
    ///
    /// - Strategy = `none`, `random` or `nomination`: The allow_list acts as pool of participants which can
    ///   be selected (by nomination or randomly etc).
    ///
    /// - Strategy = `playlist` The allow_list does not get used by this strategy.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub allow_list: Option<Vec<ParticipantId>>,

    /// Ordered list of queued participants
    ///
    /// - Strategy = `none`, `random` or `nomination`: The playlist does not get used by these strategies.
    ///
    /// - Strategy = `playlist` The playlist is a ordered list of participants which will get used to select
    ///   the next participant when yielding. It is also used as a pool to select participants
    ///   randomly from (moderator command `Select`).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub playlist: Option<Vec<ParticipantId>>,
}
