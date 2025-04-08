use serde_json::Value;

/// Returns a [Vec] containing [String]s representing possible paths
/// on JSON data
///
/// Examples:
/// ```rust
/// use serde_json::json;
/// use nestac::json::get_paths;
///
/// fn main() {
///     let json_body = json!({
///         "foo": {
///             "bar": "bingo!"
///         },
///         "hello": {
///             "world": "!"
///         }
///     });
///     let paths: Vec<String> = get_paths(
///         &json_body,
///     );
///     assert_eq!(paths.len(), 4);
///     assert_eq!(paths[0], "hello");
///     assert_eq!(paths[1], "hello.world");
///     assert_eq!(paths[2], "foo");
///     assert_eq!(paths[3], "foo.bar");
/// }
/// ```
pub fn get_paths(value: &Value) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let mut queue: Vec<(Option<String>, &Value)> = vec![(None, value)];

    while let Some((curr_path, current_value)) = queue.pop() {
        match curr_path.as_ref() {
            Some(p) => paths.push(p.clone()),
            None => (),
        };

        match current_value {
            Value::Object(map) => {
                for (key, val) in map.iter() {
                    let next_path = match &curr_path {
                        Some(p) => format!("{}.{}", p, key),
                        None => format!("{}", key),
                    };
                    queue.push((Some(next_path), val));
                }
            }
            Value::Array(arr) => match curr_path {
                Some(p) => {
                    for (index, val) in arr.iter().enumerate() {
                        let next_path = format!("{}.{}", p, index);
                        queue.push((Some(next_path), val));
                    }
                }
                None => (),
            },
            _ => {} // Skip non-object/array values
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn key_path_interpolation() {
        let json_body = json!({
            "foo": {
                "bar": {
                    "hello": "world!"
                }
            },
            "one": {
                "two": {
                    "three": {
                        "four": "five"
                    }
                }
            }
        });
        let paths: Vec<String> = get_paths(&json_body);
        assert_eq!(paths.len(), 7);
        assert_eq!(paths[0], "one");
        assert_eq!(paths[1], "one.two");
        assert_eq!(paths[2], "one.two.three");
        assert_eq!(paths[3], "one.two.three.four");
        assert_eq!(paths[4], "foo");
        assert_eq!(paths[5], "foo.bar");
        assert_eq!(paths[6], "foo.bar.hello");

        let json_body = json!({
            "medabots": {
                "hokusho": {
                    "medal": "not kabuto"
                },
                "metabee": {
                    "medal": "kabuto"
                }
            }
        });
        let paths: Vec<String> = get_paths(&json_body);
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[0], "medabots");
        assert_eq!(paths[1], "medabots.metabee");
        assert_eq!(paths[2], "medabots.metabee.medal");
        assert_eq!(paths[3], "medabots.hokusho");
        assert_eq!(paths[4], "medabots.hokusho.medal");
    }
}
