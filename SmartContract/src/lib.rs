#![feature(arbitrary_enum_discriminant)]

pub mod contract_parameter;
pub mod find_options;
pub mod trigger_type;
pub mod call_flags;
pub mod op_code;
pub mod stack_item;
pub mod script_builder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
