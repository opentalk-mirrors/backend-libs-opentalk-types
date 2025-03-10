// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// Fields that are provided when issuing the yield message
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Yield {
    /// In some cases a user must select the next participant to be speaker
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub next: Option<ParticipantId>,
}
