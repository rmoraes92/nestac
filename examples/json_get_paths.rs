use serde_json::json;
use nestac::json::get_paths;

fn main() {
    let json_body = json!({
        "foo": {
            "bar": "bingo!"
        },
        "hello": {
            "world": "!"
        }
    });
    let paths: Vec<String> = get_paths(
        &json_body,
    );
    assert_eq!(paths.len(), 5);
    assert_eq!(paths[0], "$");
    assert_eq!(paths[1], "$.hello");
    assert_eq!(paths[2], "$.hello.world");
    assert_eq!(paths[3], "$.foo");
    assert_eq!(paths[4], "$.foo.bar");
}
