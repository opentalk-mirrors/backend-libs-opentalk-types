// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

use opentalk_types_common_identifiers::module_id::ModuleId;

/// A trait for providing example data of an item.
pub trait ExampleData {
    /// Get an example instance of the current datatype.
    fn example_data() -> Self;
}

impl ExampleData for ModuleId {
    fn example_data() -> Self {
        ModuleId::example_data()
    }
}

#[cfg(test)]
mod tests {
    use opentalk_types_common_identifiers::module_id::ModuleId;

    #[test]
    fn module_id_example_data() {
        assert_eq!(
            ModuleId::example_data(),
            <ModuleId as super::ExampleData>::example_data()
        );
    }
}
