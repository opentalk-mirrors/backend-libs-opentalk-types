// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common::users::{GroupId, GroupName};

use crate::state::ChatChunk;

/// Group chat history
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupHistory {
    /// Unique id of the group
    pub id: GroupId,

    /// Name of the group
    pub name: GroupName,

    /// Group chat history chunk
    pub history: ChatChunk,
}
