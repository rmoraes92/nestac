//! # Nestac
//! _(short for (ne)sted (st)ructure (ac)cess)_ is library to access nested
//! structures using path-like string format.
//! 
//! If you work with Python you're probably familiar with
//! [glom](https://glom.readthedocs.io/en/latest/) and that is where the ideia
//! came from.
//! 
//! The necessity to make an implementation in Rust comes from a project
//! initially developed in Python that had to loop through of .json files to
//! update their properties using
//! [glom](https://glom.readthedocs.io/en/latest/).
//! 
//! Once we start increase the workload to 1k+ inputs the Python script started
//! to present performance issues. That lead to the decision of rewrite the
//! application using Rust but it would still need to support the path strings
//! to make the property updates like.
//! 
//! So here we are.

use serde_json::Value;
use regex::Regex;

/// Return a [Value] based of the dot-based [str] path you passed
///
/// # Example:
/// ```rust
/// use serde_json::Value;
/// use nestac::json_read;
/// 
/// fn main() {
///     let key_path = "foo.bar";
///     let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
///     let json_data: Value = serde_json::from_str(json_str).unwrap();
///     let val: Option<&Value> = json_read(key_path, &json_data);
///     assert_eq!(val.unwrap(), "bingo!");
/// }
/// ```
pub fn json_read<'a>(path: &str, data: &'a Value) -> Option<&'a Value> {
    let tokens = path.split(".").collect::<Vec<&str>>();
    let re_vec_idx = Regex::new(r"^\[(\d+)\]$").unwrap();
    let mut sel_data = Some(data);

    for token in tokens {
        let vec_idx = match re_vec_idx.captures(token) {
            Some(cap) => Some(cap[1].parse::<usize>().unwrap()),
            _ => None,
        };
        sel_data = match sel_data {
            Some(value) => match vec_idx {
                Some(idx) => value.get(idx),
                None => value.get(token),
            },
            None => None,
        };
    }

    return sel_data;
}

/// Update a [Value] based of the dot-based [str] path you passed
///
/// # Example:
/// ```rust
/// use serde_json::Value;
/// use nestac::{json_update, json_read};
/// 
/// fn main() {
///     let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
/// 
///     let mut json_data: Result<Value, _> = serde_json::from_str(json_str);
/// 
///     assert_eq!(json_data.is_ok(), true);
/// 
///     let old_val = json_update(
///         json_data.as_mut().unwrap(),
///         "foo.bar",
///         Value::String("updated!".into())
///     );
/// 
///     assert_eq!(old_val.is_none(), false);
///     assert_eq!(old_val.unwrap(), "bingo!");
/// 
///     let new_val: Option<&Value> = json_read(
///         "foo.bar",
///         json_data.as_ref().unwrap()
///     );
///     assert_eq!(new_val.is_none(), false);
///     assert_eq!(new_val.unwrap(), "updated!");
/// }
/// ```
pub fn json_update<'a>(data: &'a mut Value, path: &str, new_value: Value) -> Option<Value> {
    let mut tokens = path.split(".").peekable();
    let mut sel_data = Some(data);
    while let Some(token) = tokens.next() {
        if tokens.peek().is_none() {
            // last token
            return sel_data.unwrap()
                .as_object_mut()
                .unwrap()
                .insert(
                    token.to_string(),
                    new_value,
                );
        }
        sel_data = sel_data.unwrap().get_mut(token);
    }
    sel_data.cloned()
}

pub fn json_get_paths2(data: &Value, symbol: Option<String>) -> Vec<String> {
    let symbol = symbol.unwrap_or("$".to_string());
    let mut ret: Vec<String> = vec![];
    if data.is_object() {
        ret.push(symbol.clone());
        for key_s in data.as_object().unwrap().keys() {
            let child = data.as_object().unwrap().get(key_s).unwrap();
            for path in json_get_paths2(child, Some(key_s.to_string())) {
                ret.push([symbol.clone(), path].join("."));
            }
        }
    } else {
        ret.push(symbol.clone());
    }
    println!("{} - {:?}", &symbol, &ret);
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;

    #[test]
    fn update_root_key_json() {
        let json_str = r#"
            {
                "foo": "bingo!"
            }
        "#;
        let mut json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let old_val = json_update(
            json_data.as_mut().unwrap(),
            "foo",
            Value::String("updated!".into())
        );
    
        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap(), "bingo!");
    
        let new_val: Option<&Value> = json_read(
            "foo",
            json_data.as_ref().unwrap()
        );
        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap(), "updated!");
    }

    #[test]
    fn update_inner_key_json() {
        let json_str = r#"
            {
                "foo": {
                    "bar": "bingo!"
                }
            }
        "#;
        let mut json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let old_val = json_update(
            json_data.as_mut().unwrap(),
            "foo.bar",
            Value::String("updated!".into())
        );
    
        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap(), "bingo!");
    
        let new_val: Option<&Value> = json_read(
            "foo.bar",
            json_data.as_ref().unwrap()
        );
        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap(), "updated!");
    }

    #[test]
    fn read_flat_json() {
        let json_str = r#"
            {
                "foo": "bar"
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> = json_read(
            "foo",
            json_data.as_ref().unwrap()
        );
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bar");
    }

    #[test]
    fn read_inner_key_json() {
        let json_str = r#"
            {
                "foo": {
                    "bar": "bingo!"
                }
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> = json_read(
            "foo.bar",
            json_data.as_ref().unwrap()
        );
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn read_inner_array_json() {
        let json_str = r#"
            {
                "foo": [
                    "bingo!"
                ]
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> = json_read(
            "foo.[0]",
            json_data.as_ref().unwrap()
        );
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn read_inner_key_from_inner_list_json() {
        let json_str = r#"
            {
                "foo": [
                    {
                        "bar": "bingo!"
                    }
                ]
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> = json_read(
            "foo.[0].bar",
            json_data.as_ref().unwrap()
        );
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn key_path_interpolation() {
        let json_str = r#"
            {
                "foo": {
                    "bar": "bingo!"
                },
                "hello": {
                    "world": "!"
                }
            }
        "#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        let paths: Vec<String> = json_get_paths2(
            json_data.as_ref().unwrap(), None);
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[0], "$");
        assert_eq!(paths[1], "$.foo");
        assert_eq!(paths[2], "$.foo.bar");
        assert_eq!(paths[3], "$.hello");
        assert_eq!(paths[4], "$.hello.world");
    }
}
