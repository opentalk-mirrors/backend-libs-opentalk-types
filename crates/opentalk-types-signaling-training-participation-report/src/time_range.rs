// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use std::num::NonZero;

/// A time range within which checkpoints can be randomly created
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TimeRange {
    /// The earliest number of seconds after which the checkpoint can be created.
    pub after: NonZero<u64>,

    /// The number of seconds within which the checkpoint can be created after the `after` value.
    pub within: u64,
}
