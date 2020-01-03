use std::fs;
use std::process::{Command, Stdio};
use yaml_rust::YamlLoader;

fn main() {
    println!("Hello, world!");

    let content = fs::read_to_string("shrapnel.yml").expect("cannot read the file");

    println!("Content:\n{}", content);

    let parsed_content = YamlLoader::load_from_str(&content).unwrap();

    let fragments = &parsed_content[0]["fragments"];

    println!("Parsed content:\n{:?}", fragments);

    for (index, fragment) in fragments.as_vec().unwrap().iter().enumerate() {
        println!("Hello: {:?}, {:?}", index, fragment);

        let name = &fragment["name"].as_str().unwrap_or("1234");
        let dir = &fragment["dir"].as_str().unwrap_or(".");
        let cmd = &fragment["cmd"].as_str().unwrap_or("");

        println!("Name: {:?}", name);
        println!("Dir: {:?}", dir);
        println!("Cmd: {:?}", cmd);

        create_command(cmd)
            .current_dir(dir)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Something went wrong, sorry");
    }
}

fn create_command(command: &str) -> Command {
    let words: Vec<&str> = command.split_whitespace().collect();
    let fst = words[0];
    let args = &words[1..];

    let mut cmd = Command::new(fst);

    cmd.args(args);

    cmd
}
