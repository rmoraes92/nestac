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
