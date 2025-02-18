// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// The reason why presence logging started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum PresenceLoggingStartedReason {
    /// Automatically started as configured for the meeting
    Autostart,

    /// Presence logging was enabled by creator when alone in the room, and
    /// started after the first participant joined.
    FirstParticipantJoined,

    /// The creator started presence logging manually while other participants
    /// were already present in the room.
    StartedManually,
}
