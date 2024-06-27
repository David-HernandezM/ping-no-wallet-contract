use std::fs::File;
use sails_idl_gen::program;

use ping_sails_app::PingProgram;

fn main() {
    gwasm_builder::build();

    let idl_file_path = "./ping_sails.idl";
    let idl_file = File::create(idl_file_path).unwrap();

    program::generate_idl::<PingProgram>(idl_file).unwrap(); 
}