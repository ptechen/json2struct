extern crate heck;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use heck::CamelCase;
use serde_json::{{Value}};

static mut OMITEMPTY: &str = ",omitempty";

pub fn set_omitempty_empty() {
    unsafe {
        OMITEMPTY = "";
    }
}

pub fn golang_parse(params: &Value, struct_name: &String) -> String {
    let struct_header = format!("type {} struct {}", struct_name, "{");
    let mut fields: Vec<String> = vec![];
    let mut new_struct = String::new();
    if params.is_object() {
        let cur_res = is_object(params);
        fields = cur_res.0;
        fields.sort();
        new_struct = cur_res.1;
    }
    let res = format!("{}\n{}\n{}\n{}", struct_header, fields.join("\n"), "}\n", new_struct);
    return res;
}

fn is_object(params: &Value) -> (Vec<String>, String) {
    let mut fields: Vec<String> = vec![];
    let cur_map = params.as_object().unwrap();
    let mut new_struct = String::new();
    for key_val in cur_map.iter() {
        let key = key_val.0;
        let val = key_val.1;
        let cur_type = get_data_type(val, key);
        let came_key = key.as_str().to_camel_case();
        if val.is_object() {
            let cur_struct = golang_parse(val, &came_key);
            new_struct = new_struct + cur_struct.as_str()
        } else if val.is_array() {
            if cur_type.contains("*") {
                let cur_val = is_array(val);
                new_struct = golang_parse(cur_val, &came_key)
            }
        }
        unsafe {
            let field = format!("    {} {} `json:\"{}{}\"`", came_key, cur_type, key, OMITEMPTY);
            fields.push(field);
        }

    }
    return (fields, new_struct);
}

fn is_array(params: &Value) -> &Value {
    let cur = params.as_array().unwrap();
    let val = cur.get(0).unwrap();
    return val;
}

fn get_data_type(params: &Value, key: &String) -> String {
    if params.is_object() {
        let cur_type = format!("*{}", key.as_str().to_camel_case());
        return cur_type;
    } else if params.is_string() {
        let cur_type = String::from("string");
        return cur_type;
    } else if params.is_i64() {
        let cur_type = String::from("int");
        return cur_type;
    } else if params.is_boolean() {
        let cur_type = String::from("bool");
        return cur_type;
    } else if params.is_array() {
        let values = params.as_array().unwrap();
        let first = values.get(0).unwrap();
        let cur = get_data_type(first, key);
        let cur_type = format!("[]{}", cur);
        return cur_type;
    } else if params.is_f64() {
        let cur_type = String::from("float64");
        return cur_type;
    } else if params.is_u64() {
        let cur_type = String::from("uint");
        return cur_type;
    } else {
        let cur_type = String::from("interface{}");
        return cur_type;
    }
}

