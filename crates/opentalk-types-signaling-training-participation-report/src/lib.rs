// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling data types for the OpenTalk training-participation-report module.

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

pub mod command;
pub mod event;
pub mod state;

use opentalk_types_common::modules::{ModuleId, module_id};

/// The module id for the signaling module
pub const MODULE_ID: ModuleId = module_id!("training_participation_report");

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{MODULE_ID, module_id};

    #[test]
    fn verify_module_id() {
        assert_eq!(
            env!("CARGO_CRATE_NAME"),
            &format!("opentalk_types_signaling_{MODULE_ID}")
        );
        assert_eq!(MODULE_ID, module_id!("training_participation_report"));
    }
}
