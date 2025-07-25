// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Segment type for the `transcription` module

use opentalk_types_common::time::Timestamp;
use opentalk_types_signaling::ParticipantId;

/// Transcription segment
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment {
    /// Participant which the segment belongs to
    pub participant_id: ParticipantId,

    /// LiveKit track id the segment belongs to
    pub track_id: String,

    /// Segment start timestamp
    pub starts_at: Timestamp,

    /// Segment end timestamp
    pub ends_at: Timestamp,

    /// Transcribed text for the segment
    pub text: String,
}
