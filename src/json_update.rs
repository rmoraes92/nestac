use serde_json::Value;

/// Update a [Value] based off the token-based [str] path and returns a clone
/// of the old [Value] 
/// 
/// # Examples:
/// - Updating a JSON data using the default token-separator: `.`
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
///         None,
///         Value::String("updated!".into()),
///     );
/// 
///     assert_eq!(old_val.is_none(), false);
///     assert_eq!(old_val.unwrap(), "bingo!");
/// 
///     let new_val: Option<&Value> = json_read(
///         "foo.bar",
///         json_data.as_ref().unwrap(),
///         None,
///     );
///     assert_eq!(new_val.is_none(), false);
///     assert_eq!(new_val.unwrap(), "updated!");
/// }
/// ```
/// - Updating a JSON data using a custom token-separator: `@`
/// ```rust
/// use serde_json::Value;
/// use nestac::{json_update, json_read};
/// 
/// fn main() {
///     let json_str = r#"{"networks": {"192.168.0.1": "bingo!"}}"#;
/// 
///     let mut json_data: Result<Value, _> = serde_json::from_str(json_str);
/// 
///     assert_eq!(json_data.is_ok(), true);
/// 
///     let old_val = json_update(
///         json_data.as_mut().unwrap(),
///         "networks@192.168.0.1",
///         Some("@"),
///         Value::String("updated!".into()),
///     );
/// 
///     assert_eq!(old_val.is_none(), false);
///     assert_eq!(old_val.unwrap(), "bingo!");
/// 
///     let new_val: Option<&Value> = json_read(
///         "networks@192.168.0.1",
///         json_data.as_ref().unwrap(),
///         Some("@"),
///     );
///     assert_eq!(new_val.is_none(), false);
///     assert_eq!(new_val.unwrap(), "updated!");
/// }
/// ```
pub fn json_update<'a>(data: &'a mut Value, path: &str, separator: Option<&str>, new_value: Value) -> Option<Value> {
    let mut tokens = path.split(separator.unwrap_or(".")).peekable();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_read;
    use string_from::Str;
    use serde_json::Result;

    #[test]
    fn update_root_key_json() {
        let json_keypath = "foo";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": "bingo!"}"#;
        let mut json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let old_val = json_update(
            json_data.as_mut().unwrap(),
            json_keypath,
            json_separator,
            Value::String(Str!("updated!"))
        );
    
        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap(), "bingo!");
    
        let new_val: Option<&Value> = json_read(
            json_keypath,
            json_data.as_ref().unwrap(),
            json_separator,
        );
        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap(), "updated!");
    }

    #[test]
    fn update_inner_key_json() {
        let json_keypath = "foo.bar";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
        let mut json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let old_val = json_update(
            json_data.as_mut().unwrap(),
            json_keypath,
            json_separator,
            Value::String(Str!("updated!"))
        );
    
        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap(), "bingo!");
    
        let new_val: Option<&Value> = json_read(
            json_keypath,
            json_data.as_ref().unwrap(),
            json_separator,
        );
        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap(), "updated!");
    }

    #[test]
    fn update_with_custom_separator() {
        let json_keypath = "foo@192.168.0.1";
        let json_separator: Option<&str> = Some("@");
        let json_str = r#"{"foo": {"192.168.0.1": "bingo!"}}"#;
        let mut json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let old_val = json_update(
            json_data.as_mut().unwrap(),
            json_keypath,
            json_separator,
            Value::String(Str!("updated!"))
        );
    
        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap(), "bingo!");
    
        let new_val: Option<&Value> = json_read(
            json_keypath,
            json_data.as_ref().unwrap(),
            json_separator,
        );
        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap(), "updated!");
    }
}