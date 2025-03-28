use regex::Regex;
use string_from::Str;
use toml::{map::Map, Value};

/// Update a [Value] based off the token-based [str] path and returns a clone
/// of the old [Value]
///
/// # Examples:
/// - Updating a TOML data using the default token-separator: `.`
/// ```rust
/// use nestac::{toml_read, toml_update};
/// use toml::{toml, Value};
///
/// fn main() {
///     let mut toml_body = toml!(
///         [foo]
///         bar = "bingo!"
///     );
///
///     let old_val = toml_update(
///         &mut toml_body,
///         "foo.bar",
///         None,
///         Value::String("updated!".into()),
///     );
///
///     assert_eq!(old_val.is_none(), false);
///     assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");
///
///     let new_val: Option<&Value> = toml_read("foo.bar", &toml_body, None);
///     assert_eq!(new_val.is_none(), false);
///     assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
/// }
/// ```
/// - Updating a TOML data using a custom token-separator: `@`
/// ```rust
/// use nestac::{toml_read, toml_update};
/// use toml::{toml, Value};
///
/// fn main() {
///     let mut toml_body = toml!(
///         [foo]
///         bar = "bingo!"
///     );
///     let sep = Some("@");
///     let old_val = toml_update(
///         &mut toml_body,
///         "foo@bar",
///         sep,
///         Value::String("updated!".into()),
///     );
///
///     assert_eq!(old_val.is_none(), false);
///     assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");
///
///     let new_val: Option<&Value> = toml_read("foo.bar", &toml_body, None);
///     assert_eq!(new_val.is_none(), false);
///     assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
/// }
/// ```
pub fn update<'a>(
    data: &'a mut Map<String, Value>,
    path: &str,
    separator: Option<&str>,
    new_value: Value,
) -> Option<Value> {
    let mut tokens = path.split(separator.unwrap_or(".")).peekable();

    // The JSON library returns a "Value" struct as the "root node" but TOML
    // does not. Instead we have a regular Map as the "root node".
    // So that forced me to create this pre-loop step to handle the
    // initialization of the "Value" buffer (self_data).
    //
    // In case we only have a single token we don't need to loop. We simply
    // accepts the Map and apply the mutations required.
    // If we have two or more tokens then we set the self_data buffer and let
    // the main loop happens.
    //
    // We only need to worry about the tables because TOML always start with a
    // map format. Arrays/Vectors will only "show up" inside a map key.
    // To manipulate that the caller needs to pass at least two tokens.

    let mut sel_data: Option<&mut Value> = match tokens.next() {
        Some(token) => {
            if tokens.peek().is_none() {
                return data.insert(Str!(token), new_value);
            } else {
                data.get_mut(token)
            }
        }
        None => return None,
    };

    let re_vec_idx = Regex::new(r"^\[(\d+)\]$").unwrap();

    while let Some(token) = tokens.next() {
        if tokens.peek().is_none() {
            return match re_vec_idx.captures(token) {
                Some(cap) => {
                    let idx = cap[1].parse::<usize>().unwrap();
                    let tmp = sel_data.unwrap().as_array_mut().unwrap();
                    let val = tmp[idx].clone();
                    tmp[idx] = new_value;
                    Some(val)
                }
                None => sel_data
                    .unwrap()
                    .as_table_mut()
                    .unwrap()
                    .insert(Str!(token), new_value.clone()),
            };
        }
        sel_data = match re_vec_idx.captures(token) {
            Some(cap) => {
                let idx = cap[1].parse::<usize>().unwrap();
                sel_data.unwrap().as_array_mut().unwrap().get_mut(idx)
            }
            None => sel_data.unwrap().get_mut(token),
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toml::read;
    use string_from::Str;
    use toml::toml;

    #[test]
    fn update_root_key_toml() {
        let mut toml_body = toml! {
            foo = "bingo!"
        };
        let keypath = "foo";
        let sep: Option<&str> = None;
        let new_val = Value::String(Str!("updated!"));

        let old_val = update(&mut toml_body, keypath, sep, new_val);

        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

        let new_val: Option<&Value> = read(keypath, &toml_body, sep);

        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
    }

    #[test]
    fn update_root_vec_toml() {
        let mut toml_body = toml! {
            foo = ["bingo!"]
        };
        let keypath = "foo.[0]";
        let sep: Option<&str> = None;
        let new_val = Value::String(Str!("updated!"));

        let old_val = update(&mut toml_body, keypath, sep, new_val);

        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

        let new_val: Option<&Value> = read(keypath, &toml_body, sep);

        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
    }

    #[test]
    fn update_deep_key_toml() {
        let mut toml_body = toml! {
            [foo]
            bar = "bingo!"
        };
        let keypath = "foo.bar";
        let sep: Option<&str> = None;
        let new_val = Value::String(Str!("updated!"));

        let old_val = update(&mut toml_body, keypath, sep, new_val);

        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

        let new_val: Option<&Value> = read(keypath, &toml_body, sep);

        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");

        let mut toml_body = toml! {
            [foo]
            bar = {doo = "bingo!"}
        };
        let keypath = "foo.bar.doo";
        let sep: Option<&str> = None;
        let new_val = Value::String(Str!("updated!"));

        let old_val = update(&mut toml_body, keypath, sep, new_val);

        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

        let new_val: Option<&Value> = read(keypath, &toml_body, sep);

        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
    }

    #[test]
    fn update_deep_key_vec_toml() {
        let mut toml_body = toml! {
            [foo]
            bar = ["bingo!"]
        };
        let keypath = "foo.bar.[0]";
        let sep: Option<&str> = None;
        let new_val = Value::String(Str!("updated!"));

        let old_val = update(&mut toml_body, keypath, sep, new_val);

        assert_eq!(old_val.is_none(), false);
        assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

        let new_val: Option<&Value> = read(keypath, &toml_body, sep);

        assert_eq!(new_val.is_none(), false);
        assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
    }
}
