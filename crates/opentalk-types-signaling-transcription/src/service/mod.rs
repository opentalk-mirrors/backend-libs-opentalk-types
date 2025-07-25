// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling data types for the OpenTalk `transcription-service` module.

use opentalk_types_common::modules::{module_id, ModuleId};

mod command;
mod event;

pub use command::TranscriptionServiceCommand;
pub use event::TranscriptionServiceEvent;

/// The namespace string for the signaling module
pub const MODULE_ID: ModuleId = module_id!("transcription_service");
