use std::process::Command;
use anyhow::{Result, Ok};
use log::debug;
pub fn call(cmd: &str) -> Result<String> {
    debug!("exec command: {cmd}");
    let cmd_list: Vec<&str> = cmd.split_whitespace().collect();
    if cmd_list.is_empty() {
        return Err(anyhow::format_err!("invalid command: {cmd}"));
    }

    let output = Command::new(cmd_list[0])
        .args(&cmd_list[1..])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8(output.stderr)?;
        return Err(anyhow::format_err!("command \"{}\" failed. error:\"{}\"", cmd, error));
    }

    let output = String::from_utf8(output.stdout)?;

    Ok(output)
}

pub fn get_sg() -> Result<String> {
    let mut sg_device = String::new();
    let output = call("lsscsi -g")?;
    for line in output.lines() {
        if line.contains("enclosu") {
            let sg: Vec<&str> = line.split_whitespace().collect();
            if sg.is_empty() {
                return Err(anyhow::format_err!("no enclosure found!"));
            }
            let sg = sg.last().ok_or(anyhow::format_err!("no enclosure found!"))?;
            if !sg.starts_with("/dev/sg") {
                return Err(anyhow::format_err!("error enclosure info:{line}"));
            }
            
            sg_device.push_str(*sg);
            break;
        }
    }

    if sg_device.is_empty() {
        return Err(anyhow::format_err!("no enclosure found!"));
    }

    Ok(sg_device)
}