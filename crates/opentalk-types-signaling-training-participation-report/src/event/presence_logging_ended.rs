// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use super::PresenceLoggingEndedReason;

/// Event sent to participants when logging has started.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PresenceLoggingEnded {
    /// The reason why presence logging ended.
    pub reason: PresenceLoggingEndedReason,
}
