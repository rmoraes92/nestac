# nestac

_(short for (ne)sted (st)ructure (ac)cess)_ is library to access nested
structures using path-like string format.

If you work with Python you're probably familiar with
[glom](https://glom.readthedocs.io/en/latest/) and that is where the ideia
came from.

The necessity to make an implementation in Rust comes from a project initially
developed in Python that had to loop through of .json files to update their
properties using [glom](https://glom.readthedocs.io/en/latest/).

Once we start increase the workload to 1k+ inputs the Python script started to
present performance issues. That lead to the decision of rewrite the
application using Rust but it would still need to support the path strings
to make the property updates like.

So here we are.

## Installation

- `cargo add nestac`

- `cargo add --git https://github.com/mitternacht92/nestac`

## Usage

- reading nested json value

```rust
use serde_json::Value;
use nestac::json_read;

fn main() {
    let key_path = "foo.bar";
    let json_str = r#"{"foo": {"bar": "bingo!"}}"#;
    let json_data: Value = serde_json::from_str(json_str).unwrap();
    let val: Option<&Value> = json_read(key_path, &json_data);
    assert_eq!(val.unwrap(), "bingo!");
}
```

- updating nested json value

```rust
use serde_json::Value;
use nestac::json_read;

fn main() {
    let json_str = r#"{"foo": {"bar": "bingo!"}}"#;

    let mut json_data: Result<Value> = serde_json::from_str(json_str);
    assert_eq!(json_data.is_ok(), true);

    let old_val = json_update(
        json_data.as_mut().unwrap(),
        "foo.bar",
        Value::String("updated!".into())
    );

    assert_eq!(old_val.is_none(), false);
    assert_eq!(old_val.unwrap(), "bingo!");

    let new_val: Option<&Value> = json_read(
        "foo.bar",
        json_data.as_ref().unwrap()
    );

    assert_eq!(new_val.is_none(), false);
    assert_eq!(new_val.unwrap(), "updated!");
}
```

## Supported Structures

- json

## License

The MIT License (MIT)

Copyright © 2024 Ramon Moraes <mitternacht92.pub@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
