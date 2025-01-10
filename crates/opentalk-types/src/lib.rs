// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

/*!
Types and traits for the OpenTalk API and signaling protocols.

This crate re-exports items from the crates where they are defined.
All of these types can be used directly by importing the crate where they
are defined directly. If many types are needed in a project, this crate
could be the easier starting point though, because it serves as an index of
which crates are available.

# Crate features

In order to allow efficient usage of the referenced items, this crate
defines a flag for each of them. In addition some meta features are
available that pull in a set of dependencies.

## Meta features

### Types meta features

* **types-all** -
  Enables all features that depend on specific datatype crates, indirectly by
  enabling the **api** and the **signaling-all** features.
* **api** -
  Enables the **api-v1** feature.
* **signaling-all** -
  Enables all the signaling datatypes feature for all signaling modules.

### Functionality meta features

* **backend** -
  Should be enabled when implementing the server side of either a signaling module
  or an the API. Enables:
  * the **backend** feature of each types crate
  * **diesel**
  * **kustos**
  * **rand**
  * **redis**
  * **serde**
* **frontend** -
  Should be enabled when implemting the client side of either a signaling module
  or the API. Enables:
  * the **frontend** feature of each types crate
  * **serde**

## Type features

* **api-v1** -
  Re-exports [`opentalk-types-api-v1`](https://docs.rs/opentalk-types-api-v1) crate as [`api::v1`].
* **signaling** -
  Re-exports all content of the [`opentalk-types-signaling`](https://docs.rs/opentalk-types-signaling) crate in the [`signaling`] module.
* **signaling-breakout** -
  Re-exports [`opentalk-types-signaling-breakout`](https://docs.rs/opentalk-types-signaling-breakout) crate as [`signaling::breakout`].
* **signaling-chat** -
  Re-exports [`opentalk-types-signaling-chat`](https://docs.rs/opentalk-types-signaling-chat) crate as [`signaling::chat`].
* **signaling-control** -
  Re-exports [`opentalk-types-signaling-control`](https://docs.rs/opentalk-types-signaling-control) crate as [`signaling::control`].
* **signaling-livekit** -
  Re-exports [`opentalk-types-signaling-livekit`](https://docs.rs/opentalk-types-signaling-livekit) crate as [`signaling::livekit`].
* **signaling-meeting-notes** -
  Re-exports [`opentalk-types-signaling-meeting-notes`](https://docs.rs/opentalk-types-signaling-meeting-notes) crate as [`signaling::meeting-notes`].
* **signaling-meeting-report** -
  Re-exports [`opentalk-types-signaling-meeting-report`](https://docs.rs/opentalk-types-signaling-meeting-report) crate as [`signaling::meeting-report`].
* **signaling-moderation** -
  Re-exports [`opentalk-types-signaling-moderation`](https://docs.rs/opentalk-types-signaling-moderation) crate as [`signaling::moderation`].
* **signaling-polls** -
  Re-exports [`opentalk-types-signaling-polls`](https://docs.rs/opentalk-types-signaling-polls) crate as [`signaling::polls`].
* **signaling-recording** -
  Re-exports [`opentalk-types-signaling-recording`](https://docs.rs/opentalk-types-signaling-recording) crate as [`signaling::recording`].
* **signaling-recording-service** -
  Re-exports [`opentalk-types-signaling-recording-service`](https://docs.rs/opentalk-types-signaling-recording-service) crate as [`signaling::recording-service`].
* **signaling-shared-folder** -
  Re-exports [`opentalk-types-signaling-shared-folder`](https://docs.rs/opentalk-types-signaling-shared-folder) crate as [`signaling::shared-folder`].
* **signaling-subroom-audio** -
  Re-exports [`opentalk-types-signaling-subroom-audio`](https://docs.rs/opentalk-types-signaling-subroom-audio) crate as [`signaling::subroom-audio`].
* **signaling-timer** -
  Re-exports [`opentalk-types-signaling-timer`](https://docs.rs/opentalk-types-signaling-timer) crate as [`signaling::timer`].
* **signaling-whiteboard** -
  Re-exports [`opentalk-types-signaling-whiteboard`](https://docs.rs/opentalk-types-signaling-whiteboard) crate as [`signaling::whiteboard`].

## Functionality features

* **clap** -
  Should be enabled when implementing command-line tooling that exposes the
  types using [`clap`](https://docs.rs/clap). This allows listing the possible
  values of enumeration types using the builtin `--help` functionality of `clap`.
* **diesel** -
  Enabling this feature makes some newtypes storable in a database through the
  [`diesel`](https://docs.rs/diesel) crate. Used for implementing the server
  side with a `diesel` database backend.
* **kustos** -
  Adds some metadata to types that represent API resources, so that the `kustos`
  permission enforcement system can determine which permissions it needs to
  apply to certain endpoints when implementing the server side of the API.
* **rand** -
  Adds random generation of some datatypes such as `uuid`, so that they can
  be generated on the server side in the API endpoint implementation or inside
  signaling modules.
* **redis** -
  Adds annotations to some signaling data types so that they can be stored
  inside redis by the server side of a signaling module.
* **serde** -
  Adds [`serde`](https://docs.rs/serde/) `Serialize` and `Deserialize`
  implementations for each datatype that is sent over the network.
* **utoipa** -
  Adds [`utoipa`](https://docs.rs/utoipa/) `ToSchema` and `IntoParams`
  implementations to all types that are exposed in the OpenTalk Web API, so that
  they can be used to generate an OpenAPI specification of the Web API.
*/

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

#[cfg(feature = "common")]
pub use opentalk_types_common as common;

#[cfg(feature = "api-v1")]
pub mod api {
    /*!
    Re-exports all known API versions under the corresponding name.
    */
    pub use opentalk_types_api_v1 as v1;
}

#[cfg(feature = "signaling")]
pub mod signaling {
    /*!
    Re-exports all known signaling modules, each gated by a feature
    called `signaling-<modulename>`, e.g. `signaling-control` or
    `signaling-subroom-audio`.
    */
    pub use opentalk_types_signaling::*;
    #[cfg(feature = "signaling-breakout")]
    pub use opentalk_types_signaling_breakout as breakout;
    #[cfg(feature = "signaling-chat")]
    pub use opentalk_types_signaling_chat as chat;
    #[cfg(feature = "signaling-control")]
    pub use opentalk_types_signaling_control as control;
    #[cfg(feature = "signaling-livekit")]
    pub use opentalk_types_signaling_livekit as livekit;
    #[cfg(feature = "signaling-meeting-notes")]
    pub use opentalk_types_signaling_meeting_notes as meeting_notes;
    #[cfg(feature = "signaling-meeting-report")]
    pub use opentalk_types_signaling_meeting_report as meeting_report;
    #[cfg(feature = "signaling-moderation")]
    pub use opentalk_types_signaling_moderation as moderation;
    #[cfg(feature = "signaling-polls")]
    pub use opentalk_types_signaling_polls as polls;
    #[cfg(feature = "signaling-recording")]
    pub use opentalk_types_signaling_recording as recording;
    #[cfg(feature = "signaling-recording-service")]
    pub use opentalk_types_signaling_recording_service as recording_service;
    #[cfg(feature = "signaling-shared-folder")]
    pub use opentalk_types_signaling_shared_folder as shared_folder;
    #[cfg(feature = "signaling-subroom-audio")]
    pub use opentalk_types_signaling_subroom_audio as subroom_audio;
    #[cfg(feature = "signaling-timer")]
    pub use opentalk_types_signaling_timer as timer;
    #[cfg(feature = "signaling-whiteboard")]
    pub use opentalk_types_signaling_whiteboard as whiteboard;
}
