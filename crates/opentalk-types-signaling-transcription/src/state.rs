// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! State of the `transcription` module

/// The state of the `transcription` module.
///
/// This struct is sent to the participant in the `join_success` message
/// when they join successfully to the meeting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionState {
    /// Set to `true` when the room has an active transcription service connected
    pub transcription_is_active: bool,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModuleFrontendData for TranscriptionState {
    const NAMESPACE: Option<opentalk_types_common::modules::ModuleId> = Some(crate::MODULE_ID);
}
