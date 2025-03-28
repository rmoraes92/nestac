use nestac::{toml_read, toml_update};
use toml::{toml, Value};

fn main() {
    let mut toml_body = toml!(
        [foo]
        bar = "bingo!"
    );
    let sep = Some("@");
    let old_val = toml_update(
        &mut toml_body,
        "foo@bar",
        sep,
        Value::String("updated!".into()),
    );

    assert_eq!(old_val.is_none(), false);
    assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

    let new_val: Option<&Value> = toml_read("foo.bar", &toml_body, None);
    assert_eq!(new_val.is_none(), false);
    assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
}
