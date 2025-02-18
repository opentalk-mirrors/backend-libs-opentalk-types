// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling state for the `training_participation_report` namespace

mod participation_logging_state;
mod training_participation_report_state;

pub use participation_logging_state::ParticipationLoggingState;
pub use training_participation_report_state::TrainingParticipationReportState;
