// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Error from the `transcription` module namespace
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "error", rename_all = "snake_case")
)]
pub enum Error {
    /// The participant has insufficient permissions to perform a command
    InsufficientPermissions,

    /// Sent if the `Start` command was issued when a transcription session already running or was already requested
    AlreadyStarted,

    /// `Start` command failed due to an unknown error, with the transcription service
    UnknownStartError,

    /// Sent if the `Stop` command was issued when there was no transcription session active
    NothingToStop,
}
