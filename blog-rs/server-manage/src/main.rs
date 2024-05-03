use iconf::configs;
use std::env;
use plier;

mod ctrl;
mod service;

fn main() {
    let p = plier::files::get_current_dir_str();
    println!("{:?}", p);

    let file_path = String::from("server-manage/src/config.toml");

    unsafe {
        let res = configs::init(file_path);
        println!("{:?}", res);

        let c = configs::CONFIG.as_ref().unwrap();
        println!("{:?}", c);

        println!("{}", c.file_path());
        println!("{}", c.settings());

        let env = c.settings()["basics"]["env"].as_str().unwrap();
        let port = c.settings()["basics"]["port"].as_integer().unwrap();
        println!("env={} port={}", env, port);


        println!("env = {}", configs::get_str("basics", "env"));
        println!("port = {}", configs::get_int("basics", "port"));
    }
}
