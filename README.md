# Json 2 Struct for golang
[![Version info](https://img.shields.io/crates/v/json2struct.svg)](https://crates.io/crates/json2struct)

Install:

    cargo install json2struct

Json 2 Struct

USAGE:

    json2struct [OPTIONS] <json>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

    -o <omitempty>          是否添加 omitempty, example: json2struct '{"test":"test"}' -o false [default: true]
    -s <struct-name>        指定结构体名字, example: json2struct '{"test":"test"}' -s TTTT [default: XXX]

ARGS:

    <json>    Input a json string, example: json2struct '{"test":"test"}'
