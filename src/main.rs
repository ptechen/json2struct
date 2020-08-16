mod json2struct;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate heck;
use quicli::prelude::*;
use serde_json::{{Value}};
use structopt::StructOpt;
use json2struct::json2struct2golang::{{golang_parse, set_omitempty_empty}};

/// Json 2 Struct
#[derive(Debug, StructOpt)]
struct Cli {
    /// Input a json string, example: json2struct '{"test":"test"}'
    json: String,

    /// 是否添加 omitempty, example: json2struct '{"test":"test"}' -o false
    #[structopt(default_value = "true", short)]
    omitempty: String,
    /// 指定结构体名字, example: json2struct '{"test":"test"}' -s TTTT
    #[structopt(default_value = "XXX", short)]
    struct_name:String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let params: Value = serde_json::from_str(&args.json)?;
    let omit:&str = &args.omitempty;
    if omit != "true" {
        set_omitempty_empty()
    }
    let res = golang_parse(&params, &args.struct_name);
    println!("{}", res);
    Ok(())
}

