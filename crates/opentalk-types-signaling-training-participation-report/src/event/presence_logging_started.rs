// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::time::Timestamp;

use super::PresenceLoggingStartedReason;

/// Event sent to participants when logging has started.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PresenceLoggingStarted {
    /// Timestamp when the first checkpoint starts. Only included in messages sent to the creator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub first_checkpoint: Option<Timestamp>,

    /// The reason why presence logging started. Only included in messages sent to the creator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reason: Option<PresenceLoggingStartedReason>,
}
