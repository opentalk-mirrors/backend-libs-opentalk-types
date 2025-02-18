// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling events for the `training_participation_report` namespace

mod error;
mod pdf_asset;
mod presence_logging_ended;
mod presence_logging_ended_reason;
mod presence_logging_started;
mod presence_logging_started_reason;
mod training_participation_report_event;

pub use error::Error;
pub use pdf_asset::PdfAsset;
pub use presence_logging_ended::PresenceLoggingEnded;
pub use presence_logging_ended_reason::PresenceLoggingEndedReason;
pub use presence_logging_started::PresenceLoggingStarted;
pub use presence_logging_started_reason::PresenceLoggingStartedReason;
pub use training_participation_report_event::TrainingParticipationReportEvent;
