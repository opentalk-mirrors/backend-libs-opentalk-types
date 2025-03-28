// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Automod configuration data for 'automod' namespace

use opentalk_types_signaling::ParticipantId;

use crate::config::PublicConfig;

/// Data sent to the frontend on `join_success`, when automod is active.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AutomodState {
    /// Current public state of the automod configuration
    pub config: PublicConfig,

    /// Currently active speaker
    pub speaker: Option<ParticipantId>,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModuleFrontendData for AutomodState {
    const NAMESPACE: Option<opentalk_types_common::modules::ModuleId> = Some(crate::MODULE_ID);
}
