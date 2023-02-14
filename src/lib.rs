use typify::import_types;

import_types!(
    schema = "src/schema.2020-12.json",
    struct_builder = true,
    derives = [
        schemars::JsonSchema
    ],
);
