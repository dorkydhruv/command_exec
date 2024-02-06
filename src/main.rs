use std::process::Command;
use std::env::{self};

fn execute_cmd(cmd: &str) -> String {
    let mut mcmd = cmd.split(" ").collect::<Vec<&str>>();
    let mut cmds = vec!["-c"];
    cmds.append(&mut mcmd); // Modified line
    let res = Command::new("sh").args(cmds).output().unwrap();
    let stdout = String::from_utf8_lossy(res.stdout.as_slice());
    let stderr = String::from_utf8_lossy(res.stderr.as_slice());
    if stdout.len() > 0 {
        stdout.to_string()
    } else {
        stderr.to_string()
    }
}

fn main() {
    let arg:Vec<String> = env::args().collect();
    // println!("Hello, world!");
    if arg.len()==2{
        let res =execute_cmd(&arg[1]);
        print!("{}",res);
    }else{
        print!("[+] Usage: {} command",arg[0]);
    }
}
