// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling data types for the OpenTalk breakout module.

#![deny(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    overflowing_literals,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod associated_participant_in_other_room;
mod breakout_room;
mod participant_in_other_room;

pub mod command;
pub mod event;
pub mod state;

pub use associated_participant_in_other_room::AssociatedParticipantInOtherRoom;
pub use breakout_room::BreakoutRoom;
use opentalk_types_common::modules::{module_id, ModuleId};
pub use participant_in_other_room::ParticipantInOtherRoom;

/// The module id for the signaling module
pub const MODULE_ID: ModuleId = module_id!("breakout");
