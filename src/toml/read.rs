use regex::Regex;
use toml::{map::Map, Value};

/// Return a [Value] based off the token-based [str] path.
///
/// # Examples:
/// - Reading a TOML data using the default token-separator: `.`
/// ```rust
/// use nestac::toml_read;
/// use toml::{toml, Value};
///
/// fn main() {
///     let toml_body = toml!(
///         [foo]
///         bar = "bingo!"
///     );
///     let key_path = "foo.bar";
///     let val: Option<&Value> = toml_read(key_path, &toml_body, None);
///     assert_eq!(val.is_some(), true);
///     assert_eq!(val.unwrap().as_str().unwrap(), "bingo!");
/// }
/// ```
/// - Reading a TOML data using a custom token-separator: `@`
/// ```rust
/// use nestac::toml_read;
/// use toml::{toml, Value};
///
/// fn main() {
///     let toml_body = toml!(
///         [foo]
///         bar = "bingo!"
///     );
///     let key_path = "foo@bar";
///     let sep = Some("@");
///     let val: Option<&Value> = toml_read(key_path, &toml_body, sep);
///     assert_eq!(val.is_some(), true);
///     assert_eq!(val.unwrap().as_str().unwrap(), "bingo!");
/// }
/// ```
pub fn read<'a>(
    path: &str,
    data: &'a Map<String, Value>,
    separator: Option<&str>,
) -> Option<&'a Value> {
    let tokens = path.split(separator.unwrap_or(".")).collect::<Vec<&str>>();
    let re_vec_idx = Regex::new(r"^\[(\d+)\]$").unwrap();

    let mut sel_data = data.get(tokens[0]);

    if sel_data.is_none() {
        return None;
    }

    for token in tokens.iter().skip(1) {
        // TODO if we create a new Struct to encapsulate the "Value" from json,
        // toml we can try create a pre-extraction step to make sure we do not
        // need to ever skip the first index on the core loop
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
    use toml::toml;

    #[test]
    fn test_flat_toml() {
        let toml_body = toml! {
            [foo]
            bar = "hello"
        };
        let val = read("foo.bar", &toml_body, None);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_str().unwrap(), "hello");
    }

    #[test]
    fn test_flat_toml_custom_separator() {
        let toml_body = toml! {
            [foo]
            bar = "hello"
        };
        let sep = Some("@");
        let val = read("foo@bar", &toml_body, sep);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_str().unwrap(), "hello");
    }

    #[test]
    fn test_flat_toml_list() {
        let toml_body = toml! {
            [foo]
            bar = ["hello", "world"]
        };
        dbg!(&toml_body);
        let val = read("foo.bar.[1]", &toml_body, None);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_str().unwrap(), "world");
    }

    #[test]
    fn test_nested_toml() {
        let toml_body = toml! {
            [foo]
            [foo.bar]
            baz = "hello"
        };
        let val = read("foo.bar.baz", &toml_body, None);
        assert_eq!(val.unwrap().as_str().unwrap(), "hello");
    }

    #[test]
    fn test_nested_toml_list() {
        let toml_body = toml! {
            [foo]
            [foo.bar]
            baz = ["hello", "world"]
        };
        let val = read("foo.bar.baz.[1]", &toml_body, None);
        assert_eq!(val.unwrap().as_str().unwrap(), "world");
    }

    #[test]
    fn test_nested_toml_list_map() {
        let toml_body = toml! {
            [foo]
            [foo.bar]
            baz = ["hello", {inner = "world"}]
        };
        let val = read("foo.bar.baz.[1].inner", &toml_body, None);
        assert_eq!(val.unwrap().as_str().unwrap(), "world");
    }
}
