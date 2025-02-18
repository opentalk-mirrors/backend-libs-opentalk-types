// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// The reason why presence logging ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum PresenceLoggingEndedReason {
    /// Presence logging ended after the last participant besides the creator left the meeting.
    LastParticipantLeft,

    /// Presence logging ended because the creator left.
    CreatorLeft,

    /// Presence logging was stopped manually.
    StoppedManually,
}
