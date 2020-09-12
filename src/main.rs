mod json2struct;
extern crate serde_derive;
extern crate serde_json;
extern crate heck;
use quicli::prelude::*;
use serde_json::{{Value}};
use structopt::StructOpt;
use json2struct::golang::{golang_parse, set_omitempty_empty};
use json2struct::rust::{rust_parse, set_pub, set_derive};
use json2struct::{ApplicationArguments, Command};
use crate::json2struct::rust::set_camel;

fn main() -> CliResult {
    let args = ApplicationArguments::from_args();
    let key = args.command;
    match key {
        Command::Golang(opt) => {
            let params: Value = serde_json::from_str(&opt.json)?;
            let omit:&str = &opt.omitempty;
            if omit != "true" {
                set_omitempty_empty()
            }
            let res = golang_parse(&params, &opt.struct_name);
            println!("{}", res);
        }

        Command::Rust(opt) => {
            let params: Value = serde_json::from_str(&opt.json)?;
            let public = &opt.public;
            if public == "true" {
                set_pub(String::from("pub"))
            }
            let camel = &opt.camel;
            if camel != "false" {
                set_camel(String::from("#[allow(non_snake_case)]"))
            }
            let derive:&str = &opt.derive;
            set_derive(derive.to_string());
            let res = rust_parse(&params, &opt.struct_name);
            println!("{}", res);
        }
    };

    Ok(())
}

