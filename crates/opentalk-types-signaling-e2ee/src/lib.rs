// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use bytes::Bytes;
use opentalk_types_common::modules::{module_id, ModuleId};
use opentalk_types_signaling::ParticipantId;

pub const MODULE_ID: ModuleId = module_id!("e2ee");

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "snake_case", tag = "action")
)]
pub enum E2eeCommand {
    Invite(Invite),
    Message(MlsMessages),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Invite {
    pub invitee: ParticipantId,
    pub welcome_message: WelcomeMessage,
    /// Proposal and commit sent to the existing members of the group
    pub mls_messages: MlsMessages,
}

/// Arbitrary encrypted message. This can be any kind of MLS or application message
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MlsMessages {
    pub payload: Vec<Bytes>,
}

impl MlsMessages {
    pub fn is_valid(&self) -> bool {
        !self.payload.is_empty()
    }
}

/// Welcome and ratchet sent to the invitee
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WelcomeMessage {
    pub welcome: Bytes,
    pub ratchet_tree: Bytes,
}

impl WelcomeMessage {
    pub fn is_valid(&self) -> bool {
        !self.welcome.is_empty() && !self.ratchet_tree.is_empty()
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "message")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum E2eeEvent {
    Welcome(WelcomeMessage),
    MlsMessages(MlsMessages),
    // Can't use tuple variant here, because serde recognizes this as a newtype
    // which it can't serialize, see https://github.com/serde-rs/serde/issues/1307.
    ParticipantLeft { participant_id: ParticipantId },
    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    /// The targeted participant does not exist
    InvalidParticipantTarget,
    /// The invite is not valid
    InvalidInvite,
}
