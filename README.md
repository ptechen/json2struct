# Json 2 Struct for golang and rust.
[![Version info](https://img.shields.io/crates/v/json2struct.svg)](https://crates.io/crates/json2struct)

Install:

    cargo install json2struct

USAGE:

    json2struct <SUBCOMMAND>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:

    go      Json 2 Struct for golang
    help    Prints this message or the help of the given subcommand(s)
    rust    Json 2 Struct for rust

Json 2 Struct for golang

USAGE:

    json2struct go [OPTIONS] <json>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

    -o <omitempty>          是否添加 omitempty, example: json2struct go '{"test":"test"}' -o false [default: true]
    -s <struct-name>        指定结构体名字, example: json2struct go '{"test":"test"}' -s TTTT [default: XXX]

ARGS:

    <json>    Input a json string, example: json2struct go '{"test":"test"}'

Json 2 Struct for rust

USAGE:

    json2struct rust [OPTIONS] <json>

FLAGS:

    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

    -c <camel>              是否允许字段为驼峰 camel, example: json2struct rust '{"test":"test"}' -c true [default:
                            false]
    -d <derive>             添加 derive, example: json2struct rust '{"test":"test"}' -d '#[derive(Debug)]' [default:
                            #[derive(Debug)]]
    -p <public>             是否添加 pub, example: json2struct rust '{"test":"test"}' -p false [default: true]
    -s <struct-name>        指定结构体名字, example: json2struct rust '{"test":"test"}' -s TTTT [default: XXX]

ARGS:

    <json>    Input a json string, example: json2struct rust '{"test":"test"}'
