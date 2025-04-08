use nestac::json::get_paths;
use serde_json::json;

fn main() {
    let json_body = json!({
        "foo": {
            "bar": "bingo!"
        },
        "hello": {
            "world": "!"
        }
    });
    let paths: Vec<String> = get_paths(&json_body);
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], "hello");
    assert_eq!(paths[1], "hello.world");
    assert_eq!(paths[2], "foo");
    assert_eq!(paths[3], "foo.bar");
}
