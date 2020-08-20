#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

extern crate heck;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use heck::CamelCase;
use serde_json::{{Value}};

static mut OMITEMPTY: &str = ",omitempty";
static mut STRUCT_NAME: Vec<String> = vec![];
static mut INDEX: i8 = 0;

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

/// 对map类型的值进行处理
fn is_object(params: &Value) -> (Vec<String>, String) {
    let mut fields: Vec<String> = vec![];
    let cur_map = params.as_object().unwrap();
    let mut new_struct = String::new();
    for key_val in cur_map.iter() {
        let key = key_val.0;
        let val = key_val.1;
        let data = get_data_type(val, key);
        let cur_type = data.0;
        let ok = data.1;
        let came_key = key.as_str().to_camel_case();
        let mut cur_struct = String::new();
        if val.is_object() {
            cur_struct = is_ok(&cur_type, &came_key, val, ok)

        } else if val.is_array() {
            if cur_type.contains("*") {
                let cur_val = is_array(val);

                cur_struct = is_ok(&cur_type, &came_key, cur_val, ok)
            }
        }
        new_struct = new_struct + cur_struct.as_str();
        unsafe {
            let field = format!("    {} {} `json:\"{}{}\"`", came_key, cur_type, key, OMITEMPTY);
            fields.push(field);
        }
    }
    return (fields, new_struct);
}

fn is_ok(cur_type: &String, came_key:&String, val: &Value, ok: bool) -> String {
    if ok {
        let next_key = cur_type.as_str().to_camel_case();
        let cur_struct = golang_parse(val, &next_key);
        return cur_struct
    } else {
        let cur_struct = golang_parse(val, came_key);
        return cur_struct
    }
}

/// 对列表类型的数据进行处理
fn is_array(params: &Value) -> &Value {
    let cur = params.as_array().unwrap();
    let val = cur.get(0).unwrap();
    return val;
}

/// 获取数据类型
fn get_data_type(params: &Value, key: &String) -> (String, bool) {
    let mut ok = false;
    if params.is_object() {
        let mut cur_key = key.to_string();
        let res = key_exists(cur_key.clone(), cur_key.clone());
        cur_key = res.0;
        ok = res.1;
        let cur_type = format!("*{}", cur_key.as_str().to_camel_case());
        return (cur_type, ok);
    } else if params.is_string() {
        let cur_type = String::from("string");
        return (cur_type, ok);
    } else if params.is_i64() {
        let cur_type = String::from("int");
        return (cur_type, ok);
    } else if params.is_boolean() {
        let cur_type = String::from("bool");
        return (cur_type, ok);
    } else if params.is_array() {
        let values = params.as_array().unwrap();
        let first = values.get(0).unwrap_or(&serde_json::Value::Null);
        if first == &serde_json::Value::Null {
            let cur_type = format!("[]{}", "interface{}");
            return (cur_type, ok);
        }
        let cur = get_data_type(first, key);
        ok = cur.1;
        let cur_type = format!("[]{}", cur.0);
        return (cur_type, ok);
    } else if params.is_f64() {
        let cur_type = String::from("float64");
        return (cur_type, ok);
    } else if params.is_u64() {
        let cur_type = String::from("uint");
        return (cur_type, ok);
    } else {
        let cur_type = String::from("interface{}");
        return (cur_type, ok);
    }
}

fn key_exists(key: String, mut new_key: String) -> (String, bool) {
    let mut ok = false;
    unsafe {
        INDEX = INDEX + 1;
        let cur_key = format!("{}{}", key, INDEX);
        if STRUCT_NAME.contains(&new_key) {
            ok = true;
            let cur_res = key_exists(key, cur_key);
            new_key = cur_res.0;
        } else {
            STRUCT_NAME.push(new_key.clone())
        }
        INDEX = 0;
    }
    return (new_key, ok);
}


