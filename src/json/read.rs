use regex::Regex;
use serde_json::Value;

/// Return a [Value] based off the token-based [str] path.
///
/// # Examples:
/// - Reading a JSON data using the default token-separator: `.`
/// ```rust
/// use serde_json::Value;
/// use nestac::json::read;
///
/// fn main() {
///     let key_path = "foo.bar";
///     let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
///     let json_data: Value = serde_json::from_str(json_str).unwrap();
///     let val: Option<&Value> = read(key_path, &json_data, None);
///     assert_eq!(val.unwrap(), "bingo!");
/// }
/// ```
/// - Reading a JSON data using a custom token-separator: `@`
/// ```rust
/// use serde_json::Value;
/// use nestac::json::read;
///
/// fn main() {
///     let key_path = "foo@bar";
///     let separator = Some("@");
///     let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
///     let json_data: Value = serde_json::from_str(json_str).unwrap();
///     let val: Option<&Value> = read(key_path, &json_data, separator);
///     assert_eq!(val.unwrap(), "bingo!");
/// }
/// ```
pub fn read<'a>(
    path: &str,
    data: &'a Value,
    separator: Option<&str>,
) -> Option<&'a Value> {
    let tokens = path.split(separator.unwrap_or(".")).collect::<Vec<&str>>();
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;

    #[test]
    fn read_flat_json() {
        let json_keypath = "foo";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": "bar"}"#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> =
            read(json_keypath, json_data.as_ref().unwrap(), json_separator);
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bar");
    }

    #[test]
    fn read_inner_key_json() {
        let json_keypath = "foo.bar";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> =
            read(json_keypath, json_data.as_ref().unwrap(), json_separator);
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn read_inner_key_json_with_custom_delimiter() {
        let json_keypath = "foo|bar";
        let json_separator: Option<&str> = Some("|");
        let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> =
            read(json_keypath, json_data.as_ref().unwrap(), json_separator);
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn read_inner_array_json() {
        let json_keypath = "foo.[0]";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": ["bingo!"]}"#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> =
            read(&json_keypath, json_data.as_ref().unwrap(), json_separator);
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }

    #[test]
    fn read_inner_key_from_inner_list_json() {
        let json_keypath = "foo.[0].bar";
        let json_separator: Option<&str> = None;
        let json_str = r#"{"foo": [{"bar": "bingo!"}]}"#;
        let json_data: Result<Value> = serde_json::from_str(json_str);
        assert_eq!(json_data.is_ok(), true);
        let val: Option<&Value> =
            read(&json_keypath, json_data.as_ref().unwrap(), json_separator);
        assert_eq!(val.is_none(), false);
        assert_eq!(val.unwrap(), "bingo!");
    }
}
