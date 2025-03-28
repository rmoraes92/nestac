use nestac::toml_read;
use toml::{toml, Value};

fn main() {
    let toml_body = toml!(
        [foo]
        bar = "bingo!"
    );
    let key_path = "foo.bar";
    let val: Option<&Value> = toml_read(key_path, &toml_body, None);
    assert_eq!(val.is_some(), true);
    assert_eq!(val.unwrap().as_str().unwrap(), "bingo!");
}
