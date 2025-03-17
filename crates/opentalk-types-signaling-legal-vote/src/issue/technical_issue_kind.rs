// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/// Represents the types of technical issues that can occur during the vote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum TechnicalIssueKind {
    /// An issue related to audio during the vote.
    Audio,

    /// An issue related to video during the vote.
    Video,

    /// An issue related to screen sharing during the vote.
    Screenshare,
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialization_audio_technical_issue_kind() {
        let produced = serde_json::to_value(TechnicalIssueKind::Audio).unwrap();

        let expected = json!("audio");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_audio_technical_issue_kind() {
        let produced: TechnicalIssueKind = serde_json::from_value(json!("audio")).unwrap();

        let expected = TechnicalIssueKind::Audio;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_video_technical_issue_kind() {
        let produced = serde_json::to_value(TechnicalIssueKind::Video).unwrap();

        let expected = json!("video");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_video_technical_issue_kind() {
        let produced: TechnicalIssueKind = serde_json::from_value(json!("video")).unwrap();

        let expected = TechnicalIssueKind::Video;

        assert_eq!(produced, expected);
    }

    #[test]
    fn serialization_screenshare_technical_issue_kind() {
        let produced = serde_json::to_value(TechnicalIssueKind::Screenshare).unwrap();

        let expected = json!("screenshare");

        assert_eq!(produced, expected);
    }

    #[test]
    fn deserialization_screenshare_technical_issue_kind() {
        let produced: TechnicalIssueKind = serde_json::from_value(json!("screenshare")).unwrap();

        let expected = TechnicalIssueKind::Screenshare;

        assert_eq!(produced, expected);
    }
}
