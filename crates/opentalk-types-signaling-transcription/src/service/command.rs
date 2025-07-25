// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling command for the `transcription-service` module

use crate::segment::Segment;

/// Command for the `transcription-service` module
#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", tag = "action")
)]
pub enum TranscriptionServiceCommand {
    /// Send a new segment
    Segment(Segment),
}
