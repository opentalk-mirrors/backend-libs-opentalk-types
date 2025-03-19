// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Signaling event messages for the `automod` namespace

mod automod_event;
mod error;
mod remaining_updated;
mod speaker_updated;
mod start_animation;
mod stopped_reason;

pub use automod_event::AutomodEvent;
pub use error::Error;
pub use remaining_updated::RemainingUpdated;
pub use speaker_updated::SpeakerUpdated;
pub use start_animation::StartAnimation;
pub use stopped_reason::StoppedReason;
