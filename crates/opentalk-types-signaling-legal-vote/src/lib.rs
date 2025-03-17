// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling data types for the OpenTalk legal vote module.

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

pub mod cancel;
pub mod command;
pub mod event;
pub mod state;
pub mod token;
pub use opentalk_types_common::modules::{module_id, ModuleId};
pub mod invalid;
pub mod issue;
pub mod parameters;
pub mod tally;
pub mod user_parameters;
pub mod vote;

/// The module id for the signaling module
pub const MODULE_ID: ModuleId = module_id!("legal_vote");
