// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling event messages for the `legal-vote` namespace.

mod canceled;
mod error_kind;
mod final_results;
mod guest_participants;
mod invalid_fields;
mod legal_vote_event;
mod pdf_asset;
mod reported_issue;
mod response;
mod results;
mod stopped;
mod vote_failed;
mod vote_response;
mod vote_results;
mod vote_success;
mod voting_record;

pub use canceled::Canceled;
pub use error_kind::ErrorKind;
pub use final_results::FinalResults;
pub use guest_participants::GuestParticipants;
pub use invalid_fields::InvalidFields;
pub use legal_vote_event::LegalVoteEvent;
pub use pdf_asset::PdfAsset;
pub use reported_issue::ReportedIssue;
pub use response::Response;
pub use results::Results;
pub use stopped::Stopped;
pub use vote_failed::VoteFailed;
pub use vote_response::VoteResponse;
pub use vote_results::VoteResults;
pub use vote_success::VoteSuccess;
pub use voting_record::VotingRecord;
