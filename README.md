# nestac

_(short for (ne)sted (st)ructure (ac)cess)_ is library to access nested
structures using path-like string format.

If you work with Python you're probably familiar with
[glom](https://glom.readthedocs.io/en/latest/) and that is where the ideia came
from.

The necessity to make an implementation in Rust comes from a project initially
developed in Python that had to loop through of .json files to update their
properties using [glom](https://glom.readthedocs.io/en/latest/).

Once we start increase the workload to 1k+ inputs the Python script started to
present performance issues. That lead to the decision of rewrite the application
using Rust but it would still need to support the path strings to make the
property updates just like with [glom](https://glom.readthedocs.io/en/latest/).

## 1. Supported Structures

- json
- toml

## 2. Installation

`cargo add nestac`

## 3. Usage

### 3.1 Reading Nested Value

#### 3.1.1 JSON

```rust
use serde_json::{json, Value};
use nestac::json::read;

fn main() {
    let json_body = json!({"foo": {"bar": "bingo!"}});
    let key_path = "foo.bar";
    let val: Option<&Value> = read(key_path, &json_body, None);
    assert_eq!(val.unwrap(), "bingo!");
}
```

#### 3.1.2 TOML

```rust
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
```

### 3.2 Updating Nested Json Value

#### 3.2.1 JSON

```rust
use serde_json::{json, Value};
use nestac::json::{read, update};

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

    let new_val: Option<&Value> = read(
        "foo.bar",
        &json_body,
        None,
    );
    assert_eq!(new_val.is_none(), false);
    assert_eq!(new_val.unwrap(), "updated!");
}
```

#### 3.2.2 TOML

```rust
use nestac::{toml_read, toml_update};
use toml::{toml, Value};

fn main() {
    let mut toml_body = toml!(
        [foo]
        bar = "bingo!"
    );

    let old_val = toml_update(
        &mut toml_body,
        "foo.bar",
        None,
        Value::String("updated!".into()),
    );

    assert_eq!(old_val.is_none(), false);
    assert_eq!(old_val.unwrap().as_str().unwrap(), "bingo!");

    let new_val: Option<&Value> = toml_read("foo.bar", &toml_body, None);
    assert_eq!(new_val.is_none(), false);
    assert_eq!(new_val.unwrap().as_str().unwrap(), "updated!");
}
```

### 3.3 generate a list of possible key-paths

#### 3.3.1 JSON

```rust
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
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], "hello");
    assert_eq!(paths[1], "hello.world");
    assert_eq!(paths[2], "foo");
    assert_eq!(paths[3], "foo.bar");
}
```

#### 3.3.2 TOML

```rust
use nestac::toml_get_paths;
use toml::toml;

fn main() {
    let toml_body = toml!(
        [foo]
        bar = "bingo!"
        [hello]
        world = "!"
    );
    let paths: Vec<String> = toml_get_paths(&toml_body);
    assert_eq!(paths.len(), 4);
    assert_eq!(paths[0], "foo");
    assert_eq!(paths[1], "foo.bar");
    assert_eq!(paths[2], "hello");
    assert_eq!(paths[3], "hello.world");
}
```

## 4. Examples

- `cargo run --example json_get_paths`
- `cargo run --example json_read_value`
- `cargo run --example json_update_attribute`
- `cargo run --example json_update_attribute_custom_sep`
- `cargo run --example toml_get_paths`
- `cargo run --example toml_read_value`
- `cargo run --example toml_update_attribute`
- `cargo run --example toml_update_attributes_custom_sep`

## License

The MIT License (MIT)

Copyright © 2024 Ramon Moraes <ramonmoraes.foss@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
