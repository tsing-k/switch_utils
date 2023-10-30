use anyhow::{Ok, Result};
use dialoguer::Select;
use std::{fs, fs::File, io::Read};
use walkdir::WalkDir;

use crate::utils::{call, get_sg};

#[derive(Debug, Clone)]
pub enum DownloadFileType {
    Bootloader(String),
    Firmware(String),
    Nvdata(String),
    Userdefault(String),
    Package(String),
    Flash32M(String),
}

impl DownloadFileType {
    pub fn open(file: &str) -> Result<Self> {
        let mut f = File::open(file)?;
        let mut data = [0_u8; 24];
        let n = f.read(&mut data)?;

        if n != 24 {
            return Err(anyhow::format_err!("read file failed. only read {n} bytes"));
        }

        let magic = u32::from_le_bytes(data[0..4].try_into()?);
        let image_type = u32::from_le_bytes(data[20..24].try_into()?);
        let size = fs::metadata(file)?.len();

        match (magic, size, image_type) {
            (0x327f68cd, _, _) => Ok(DownloadFileType::Package(String::from(file))),
            (0x327f68ab, len, 2) => {
                if len == 0x2000000 {
                    Ok(DownloadFileType::Flash32M(String::from(file)))
                } else {
                    Ok(DownloadFileType::Bootloader(String::from(file)))
                }
            }
            (0x327f68ab, _, 4) => Ok(DownloadFileType::Firmware(String::from(file))),
            (0x327f68ab, _, 6) => Ok(DownloadFileType::Nvdata(String::from(file))),
            (0x327f68ab, _, 15) => Ok(DownloadFileType::Userdefault(String::from(file))),
            _ => Err(anyhow::format_err!("invalid file")),
        }
    }
}

pub fn get_file(dir: &str) -> Result<DownloadFileType> {
    let mut file_list: Vec<DownloadFileType> = vec![];
    let mut filename_list: Vec<String> = vec![];
    let walker = WalkDir::new(dir).max_depth(1).into_iter();
    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let filename = entry
            .file_name()
            .to_str()
            .ok_or(anyhow::format_err!("invalid entry"))?;
        if filename.starts_with('.') {
            continue;
        }

        let file = DownloadFileType::open(filename);
        if file.is_err() {
            continue;
        }

        let file = file.unwrap();
        file_list.push(file);
        filename_list.push(String::from(filename));
    }

    if file_list.is_empty() {
        Err(anyhow::format_err!("no available file found"))
    } else if file_list.len() == 1 {
        Ok(file_list[0].clone())
    } else {
        let mut index: i32 = -1;
        // 优先找package
        for (i, file) in file_list.iter().enumerate() {
            if let DownloadFileType::Package(_) = file {
                index = i as i32;
                break;
            }
        }

        if index < 0 {
            index = Select::new()
                .with_prompt("select file to download")
                .items(&filename_list)
                .default(0)
                .interact()? as i32;
        }

        Ok(file_list[index as usize].clone())
    }
}

pub fn download(file: DownloadFileType) -> Result<()> {
    // 检查enclosure设备是否存在
    let sg = get_sg()?;

    // 关halt
    call("ps3cli /psw0 custom pcieswitch halt off")?;

    // 关chipid check
    call("ps3cli /psw0 custom pcieswitch chipid off")?;

    match file {
        DownloadFileType::Bootloader(file) => {
            println!("downlaod bootloader: {file}");
            call(format!("ps3cli /psw0 download file={file} fwtype=2").as_str())?;
        }
        DownloadFileType::Firmware(file) => {
            println!("downlaod firmware: {file}");
            call(format!("ps3cli /psw0 download file={file} fwtype=4").as_str())?;
        }
        DownloadFileType::Nvdata(file) => {
            println!("downlaod nvdata: {file}");
            call(format!("ps3cli /psw0 download file={file} fwtype=6").as_str())?;
        }
        DownloadFileType::Userdefault(file) => {
            println!("downlaod userdefault: {file}");
            call(format!("ps3cli /psw0 download file={file} fwtype=15").as_str())?;
        }
        DownloadFileType::Package(file) => {
            println!("downlaod package: {file}");
            call(format!("ps3cli /psw0 download file={file}").as_str())?;
        }
        DownloadFileType::Flash32M(file) => {
            println!("downlaod 32M: {file}");
            call(format!("split -b 16M -d -a 1 {file} {file}_").as_str())?;
            call(format!("sg_write_buffer -m 0x1d -b 4k -l 16M {sg} -I {file}_0").as_str())?;
            call(format!("sg_write_buffer -m 0x1d -b 4k -l 16M {sg} -I {file}_1").as_str())?;
        }
    }

    Ok(())
}
