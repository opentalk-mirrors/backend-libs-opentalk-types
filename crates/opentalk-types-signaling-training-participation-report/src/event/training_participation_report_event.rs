// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling events for the `meeting_report` namespace

use super::{Error, PdfAsset, PresenceLoggingEnded, PresenceLoggingStarted};

/// Events sent out by the `meeting_report` module
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "message")
)]
pub enum TrainingParticipationReportEvent {
    /// Confirmation to the trainer that presence logging has been enabled
    PresenceLoggingEnabled,

    /// Confirmation to the trainer that presence logging has been disabled
    PresenceLoggingDisabled,

    /// Information to participants that presence logging has started
    PresenceLoggingStarted(PresenceLoggingStarted),

    /// Information to participants that presence logging has ended
    PresenceLoggingEnded(PresenceLoggingEnded),

    /// Sent to all participants as a request to confirm their presence.
    PresenceConfirmationRequested,

    /// Sent all participants as a confirmation that their presence has been logged.
    PresenceConfirmationLogged,

    /// A PDF asset has been created
    PdfAsset(PdfAsset),

    /// An error happened when executing a `meeting_report` command
    Error(Error),
}

impl From<PresenceLoggingStarted> for TrainingParticipationReportEvent {
    fn from(value: PresenceLoggingStarted) -> Self {
        Self::PresenceLoggingStarted(value)
    }
}

impl From<PresenceLoggingEnded> for TrainingParticipationReportEvent {
    fn from(value: PresenceLoggingEnded) -> Self {
        Self::PresenceLoggingEnded(value)
    }
}

impl From<PdfAsset> for TrainingParticipationReportEvent {
    fn from(value: PdfAsset) -> Self {
        Self::PdfAsset(value)
    }
}

impl From<Error> for TrainingParticipationReportEvent {
    fn from(value: Error) -> Self {
        Self::Error(value)
    }
}

#[cfg(test)]
mod tests {
    use opentalk_types_common::assets::AssetId;
    use pretty_assertions::assert_eq;

    use crate::event::{
        Error, PdfAsset, PresenceLoggingEnded, PresenceLoggingEndedReason, PresenceLoggingStarted,
        PresenceLoggingStartedReason, TrainingParticipationReportEvent,
    };

    #[test]
    fn from_presence_logging_started() {
        let msg = PresenceLoggingStarted {
            first_checkpoint: None,
            reason: Some(PresenceLoggingStartedReason::Autostart),
        };
        assert_eq!(
            TrainingParticipationReportEvent::from(msg.clone()),
            TrainingParticipationReportEvent::PresenceLoggingStarted(msg)
        );
    }

    #[test]
    fn from_presence_logging_ended() {
        let msg = PresenceLoggingEnded {
            reason: PresenceLoggingEndedReason::LastParticipantLeft,
        };
        assert_eq!(
            TrainingParticipationReportEvent::from(msg.clone()),
            TrainingParticipationReportEvent::PresenceLoggingEnded(msg)
        );
    }

    #[test]
    fn from_pdf_asset() {
        let msg = PdfAsset {
            filename: "example.pdf".into(),
            asset_id: AssetId::from_u128(0x99224411abcd),
        };
        assert_eq!(
            TrainingParticipationReportEvent::from(msg.clone()),
            TrainingParticipationReportEvent::PdfAsset(msg)
        );
    }

    #[test]
    fn from_error() {
        let msg = Error::InsufficientPermissions;
        assert_eq!(
            TrainingParticipationReportEvent::from(msg.clone()),
            TrainingParticipationReportEvent::Error(msg)
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use opentalk_types_common::assets::AssetId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::{Error, PdfAsset, TrainingParticipationReportEvent};
    use crate::event::{PresenceLoggingStarted, PresenceLoggingStartedReason};

    #[test]
    fn serialize_event_presence_logging_enabled() {
        let event = TrainingParticipationReportEvent::PresenceLoggingEnabled;
        let value = serde_json::to_value(event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "message": "presence_logging_enabled",
            })
        );
    }

    #[test]
    fn serialize_event_presence_logging_disabled() {
        let event = TrainingParticipationReportEvent::PresenceLoggingDisabled;
        let value = serde_json::to_value(event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "message": "presence_logging_disabled",
            })
        );
    }

    #[test]
    fn serialize_event_presence_logging_started() {
        let event =
            TrainingParticipationReportEvent::PresenceLoggingStarted(PresenceLoggingStarted {
                first_checkpoint: None,
                reason: None,
            });
        let value = serde_json::to_value(event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "message": "presence_logging_started",
            })
        );
    }

    #[test]
    fn serialize_event_presence_logging_started_with_optional_fields() {
        let event =
            TrainingParticipationReportEvent::PresenceLoggingStarted(PresenceLoggingStarted {
                first_checkpoint: Some(
                    "2025-02-03T04:05:06Z"
                        .parse()
                        .expect("must be parseable as timestamp"),
                ),
                reason: Some(PresenceLoggingStartedReason::StartedManually),
            });
        let value = serde_json::to_value(event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "message": "presence_logging_started",
                "first_checkpoint": "2025-02-03T04:05:06Z",
                "reason": "started_manually"
            })
        );
    }

    #[test]
    fn serialize_meeting_report_event_pdf_asset() {
        let event = TrainingParticipationReportEvent::PdfAsset(PdfAsset {
            filename: "pdf-file.pdf".to_owned(),
            asset_id: AssetId::from_u128(0x735fcdaa_56dd_4ddb_9eb0_7d083a4a9d9b),
        });
        let value = serde_json::to_value(event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "filename": "pdf-file.pdf",
                "asset_id": "735fcdaa-56dd-4ddb-9eb0-7d083a4a9d9b",
                "message": "pdf_asset",
            })
        );
    }

    #[test]
    fn serialize_meeting_report_event_error() {
        let pdf_event = TrainingParticipationReportEvent::Error(Error::StorageExceeded);
        let value = serde_json::to_value(pdf_event).expect("Must be serializable");
        assert_eq!(
            value,
            json!({
                "message": "error",
                "error": "storage_exceeded",
            })
        );
    }
}
