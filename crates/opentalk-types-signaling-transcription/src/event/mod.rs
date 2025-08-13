// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling events for the `transcription` namespace

use crate::segment::Segment;

mod error;

pub use error::Error;

/// Event from the transcription module
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "message", rename_all = "snake_case")
)]
pub enum TranscriptionEvent {
    /// Transcription of the room has started
    Started,

    /// Transcription of the room has stopped
    Stopped,

    /// A new segment has been created
    Segment(Segment),

    /// An error occurred
    Error(Error),
}
