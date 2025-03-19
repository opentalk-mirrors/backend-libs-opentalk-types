// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// A command from the frontend has triggered an error.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "error")
)]
pub enum Error {
    /// The selection made by the frontend was invalid.
    ///
    /// Can originate from the `start`, `yield` or `select` command.
    InvalidSelection,

    /// The issued command can only be issued by a moderator, but the issuer isn't one.
    InsufficientPermissions,

    /// An automod session is already running.
    SessionAlreadyRunning,
}
