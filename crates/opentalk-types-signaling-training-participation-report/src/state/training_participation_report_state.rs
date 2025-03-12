// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling state for the `training_participation_report` namespace

use opentalk_types_common::training_participation_report::TrainingParticipationReportParameterSet;

use super::ParticipationLoggingState;

/// /// The state of the `training_participation_report` module
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrainingParticipationReportState {
    /// Current state of the participation logging procedure
    pub state: ParticipationLoggingState,

    /// The default parameter set of the room. Only communicated to the room owner.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub parameter_set: Option<TrainingParticipationReportParameterSet>,
}

#[cfg(feature = "serde")]
impl opentalk_types_signaling::SignalingModuleFrontendData for TrainingParticipationReportState {
    const NAMESPACE: Option<opentalk_types_common::modules::ModuleId> = Some(crate::MODULE_ID);
}
