// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Data types for handling assets.

mod asset_file_kind;
mod asset_id;
mod asset_sorting;
mod file_extension;

pub use asset_file_kind::{AssetFileKind, MAX_ASSET_FILE_KIND_LENGTH, MIN_ASSET_FILE_KIND_LENGTH};
pub use asset_id::AssetId;
pub use asset_sorting::AssetSorting;
pub use file_extension::{FileExtension, MAX_FILE_EXTENSION_LENGTH};
