use toml::{map::Map, Value};

/// Returns a [Vec] containing [String]s representing possible paths
/// in a TOML data.
///
/// Examples:
/// ```rust
/// use nestac::toml_get_paths;
/// use toml::toml;
///
/// fn main() {
///     let toml_body = toml!(
///         [foo]
///         bar = "bingo!"
///         [hello]
///         world = "!"
///     );
///     let paths: Vec<String> = toml_get_paths(&toml_body);
///     assert_eq!(paths.len(), 4);
///     assert_eq!(paths[0], "foo");
///     assert_eq!(paths[1], "foo.bar");
///     assert_eq!(paths[2], "hello");
///     assert_eq!(paths[3], "hello.world");
/// }
/// ```
pub fn get_paths(data: &Map<String, Value>) -> Vec<String> {
    let mut paths = Vec::new();
    let mut queue: Vec<(String, &Value)> = vec![];

    for key in data.keys() {
        let val: &Value = data.get(key).unwrap();
        queue.push((key.clone(), val));
        while let Some((current_path, current_value)) = queue.pop() {
            paths.push(current_path.clone());

            match current_value {
                Value::Table(map) => {
                    for (key, val) in map.iter() {
                        let next_path = format!("{}.{}", current_path, key);
                        queue.push((next_path, val));
                    }
                }
                Value::Array(arr) => {
                    for (index, val) in arr.iter().enumerate() {
                        let next_path = format!("{}.{}", current_path, index);
                        queue.push((next_path, val));
                    }
                }
                _ => {} // Skip non-object/array values
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::toml;

    #[test]
    fn key_path_interpolation() {
        let toml_body = toml!(
            [foo]
            bar = {
                hello = "world!"
            }
            [one]
            two = {
                three = {
                    four = "five"
                }
            }
        );
        let paths: Vec<String> = get_paths(&toml_body);
        dbg!(&paths);
        assert_eq!(paths.len(), 7);
        // TODO ensure order consistency?
        assert_eq!(paths[0], "foo");
        assert_eq!(paths[1], "foo.bar");
        assert_eq!(paths[2], "foo.bar.hello");
        assert_eq!(paths[3], "one");
        assert_eq!(paths[4], "one.two");
        assert_eq!(paths[5], "one.two.three");
        assert_eq!(paths[6], "one.two.three.four");

        let toml_body = toml!(
            [medabots]
            hokusho = {
                medal = "not kabuto"
            }
            metabee = {
                medal = "kabuto"
            }
        );
        let paths: Vec<String> = get_paths(&toml_body);
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[0], "medabots");
        assert_eq!(paths[1], "medabots.metabee");
        assert_eq!(paths[2], "medabots.metabee.medal");
        assert_eq!(paths[3], "medabots.hokusho");
        assert_eq!(paths[4], "medabots.hokusho.medal");
    }
}
