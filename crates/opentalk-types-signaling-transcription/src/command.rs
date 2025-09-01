// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling commands for the `transcription` module

/// Command for the `transcription` module
#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", tag = "action")
)]
pub enum TranscriptionCommand {
    /// Start transcribing
    Start {
        /// Target language of the transcription, specified as a two-letter language code (e.g., `de`, `en`).
        ///
        /// Set to `None` for auto-detection.
        language: Option<String>,
    },

    /// Stop transcribing
    Stop,
}
