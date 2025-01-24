// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Error from the `meeting_report` module namespace
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "error", rename_all = "snake_case")
)]
pub enum Error {
    /// The requesting user has insufficient permissions for the operation
    InsufficientPermissions,

    /// The creator attempted to enable presence logging when it was already enabled.
    PresenceLoggingAlreadyEnabled,

    /// A frontend attempted to perform an action that requires enabled presence logging when it wasn't enabled.
    PresenceLoggingNotEnabled,

    /// A participant who shouldn't confirm the presence attempted to do so.
    PresenceLoggingNotAllowedForParticipant,

    /// Storage exceeded
    StorageExceeded,

    /// Internal error while generating the report
    Generate,

    /// Internal error while saving the report
    Storage,
}
