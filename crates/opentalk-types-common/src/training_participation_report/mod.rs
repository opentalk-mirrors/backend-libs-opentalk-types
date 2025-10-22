// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Data types for handling training participation reports.

mod time_range;
mod time_range_start;
mod time_range_window;
mod training_participation_report_parameter_set;

pub use time_range::TimeRange;
pub use time_range_start::TimeRangeStart;
pub use time_range_window::TimeRangeWindow;
pub use training_participation_report_parameter_set::TrainingParticipationReportParameterSet;

/// The number of seconds per minute
pub const SECONDS_PER_MINUTE: i64 = 60;
/// The number of minutes per hour
pub const MINUTES_PER_HOUR: i64 = 60;
