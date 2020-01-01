use std::fs;
use yaml_rust::YamlLoader;

fn main() {
    println!("Hello, world!");

    let content = fs::read_to_string("shrapnel.yml").expect("cannot read the file");

    println!("Content:\n{}", content);

    let parsed_content = YamlLoader::load_from_str(&content);

    println!(
        "Parsed content:\n{:?}",
        parsed_content.unwrap()[0]["fragments"]
    );
}
