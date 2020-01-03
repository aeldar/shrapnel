use std::env;
use std::fs;
use std::process::{Command, Stdio};
use yaml_rust::YamlLoader;

fn main() {
    let content = fs::read_to_string("shrapnel.yml").expect("cannot read the file");

    let parsed_content = YamlLoader::load_from_str(&content).unwrap();

    let fragments = &parsed_content[0]["fragments"];

    for (index, fragment) in fragments.as_vec().unwrap().iter().enumerate() {
        let _name = &fragment["name"]
            .as_str()
            .unwrap_or(&format!("{}", index)[..]);
        let dir = &fragment["dir"].as_str().unwrap_or(".");
        let cmd = &fragment["cmd"].as_str().unwrap_or("ls");

        create_command(cmd)
            .envs(env::vars())
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
