use std::process::Command;
use anyhow::{Result, Ok};

pub fn call(cmd: &str) -> Result<String> {
    let cmd_list: Vec<&str> = cmd.split_whitespace().collect();
    if cmd_list.len() == 0 {
        return Err(anyhow::format_err!("invalid command: {cmd}"));
    }

    let output = Command::new(cmd_list[0])
        .args(&cmd_list[1..])
        .output()?;
    
    if !output.status.success() {
        return Err(anyhow::format_err!("exec command {} failed. {}", cmd, output.status));
    }

    let output = String::from_utf8(output.stdout)?;

    Ok(output)
}

pub fn get_sg() -> Result<String> {
    let output = call("lsscsi -g | grep enc")?;
    let sg: Vec<&str> = output.split_whitespace().collect();

    if sg.len() == 0 {
        return Err(anyhow::format_err!("no enclosure found!"));
    }

    let sg = sg.last().ok_or(anyhow::format_err!("no enclosure found!"))?;
    if !sg.starts_with("/dev/sg") {
        return Err(anyhow::format_err!("no enclosure found!"));
    }

    Ok(String::from(*sg))
}