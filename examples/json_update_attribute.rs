use nestac::json::{read, update};
use serde_json::{json, Value};

fn main() {
    let mut json_body = json!({"foo": {"bar": "bingo!"}});

    let old_val = update(
        &mut json_body,
        "foo.bar",
        None,
        Value::String("updated!".into()),
    );

    assert_eq!(old_val.is_none(), false);
    assert_eq!(old_val.unwrap(), "bingo!");

    let new_val: Option<&Value> = read("foo.bar", &json_body, None);
    assert_eq!(new_val.is_none(), false);
    assert_eq!(new_val.unwrap(), "updated!");
}
