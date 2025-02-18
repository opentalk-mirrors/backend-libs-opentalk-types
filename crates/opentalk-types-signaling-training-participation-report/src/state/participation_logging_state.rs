// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// The state of the `training_participation_report` module
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum ParticipationLoggingState {
    /// No participation logging is active, nothing to do for a client.
    Disabled,

    /// Participation logging is enabled, either waiting for the initial timeout
    /// or the participant already confirmed the last checkpoint. A client
    /// should notify the participant about this state.
    Enabled,

    /// Participation logging is enabled, a checkpoint has already been passed
    /// and the newly joined participant can immediately confirm their presence.
    WaitingForConfirmation,
}
