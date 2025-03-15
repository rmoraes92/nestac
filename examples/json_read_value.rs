use serde_json::{json, Value};
use nestac::json::read;

fn main() {
    let json_body = json!({"foo": {"bar": "bingo!"}});
    let key_path = "foo.bar";
    let val: Option<&Value> = read(key_path, &json_body, None);
    assert_eq!(val.unwrap(), "bingo!");
}
