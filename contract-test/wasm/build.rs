use std::fs::File;
use sails_idl_gen::program;

use contract_test_app::MyProgram;

fn main() {
    gwasm_builder::build();

    let idl_file_path = "./ping_sails.idl";
    let idl_file = File::create(idl_file_path).unwrap();

    program::generate_idl::<MyProgram>(idl_file).unwrap(); 
}