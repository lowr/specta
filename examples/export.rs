use specta::{
    export,
    ts::{BigIntExportBehavior, ExportConfiguration},
    Type,
};

#[derive(Type)]
pub struct TypeOne {
    pub field1: String,
    pub field2: TypeTwo,
}

#[derive(Type)]
pub struct TypeTwo {
    pub my_field: String,
}

fn main() {
    // This will automatically discover all types in your project!
    export::ts("./bindings.ts").unwrap();

    // You can also override the export configuration.
    export::ts_with_cfg(
        // Be aware this won't be typesafe unless your using a ser/deserializer that converts BigInt types to a number.
        &ExportConfiguration::default().bigint(BigIntExportBehavior::Number),
        "./bindings2.ts",
    )
    .unwrap();
}
