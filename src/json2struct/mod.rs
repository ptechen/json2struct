pub mod rust;
pub mod golang;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "rust")]
    Rust(Rust),

    #[structopt(name = "go")]
    Golang(Golang),


}

/// Json 2 Struct for golang.
#[derive(Debug, StructOpt)]
pub struct Golang {
    /// Input a json string, example: json2struct '{"test":"test"}'
    pub json: String,

    /// 是否添加 omitempty, example: json2struct '{"test":"test"}' -o false
    #[structopt(default_value = "true", short)]
    pub omitempty: String,

    /// 指定结构体名字, example: json2struct '{"test":"test"}' -s TTTT
    #[structopt(default_value = "XXX", short)]
    pub struct_name:String,
}

/// Json 2 Struct for rust.
#[derive(Debug, StructOpt)]
pub struct Rust {
    /// Input a json string, example: json2struct '{"test":"test"}'
    pub json: String,

    /// 是否添加 pub, example: json2struct '{"test":"test"}' -p false
    #[structopt(default_value = "true", short)]
    pub public: String,

    /// 添加 derive, example: json2struct '{"test":"test"}' -d '#[derive(Debug)]'
    #[structopt(default_value = "#[derive(Debug)]", short)]
    pub derive: String,

    /// 是否允许字段为驼峰 camel, example: json2struct '{"test":"test"}' -c true
    #[structopt(default_value = "false", short)]
    pub camel: String,

    /// 指定结构体名字, example: json2struct '{"test":"test"}' -s TTTT
    #[structopt(default_value = "XXX", short)]
    pub struct_name:String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "classify")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,
}