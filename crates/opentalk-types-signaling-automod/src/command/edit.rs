// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_signaling::ParticipantId;

/// Fields that are provided when issuing the edit message
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Edit {
    /// Edit the `allow_list`. If `None`, it should not be edited.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub allow_list: Option<Vec<ParticipantId>>,

    /// Edit the `playlist`. If `None`, it should not be edited.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub playlist: Option<Vec<ParticipantId>>,
}
