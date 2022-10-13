use std::env;
use std::fs;
use std::fs::File;
use std::io::{copy, Read};
use std::io::Cursor;
use std::path::Path;
use reqwest;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const SERVER_PATH: &str = "config";
const SERVER_FILE: &str = "/server.json";
const SERVER_FILE_SOURCE: &str = "https://raw.githubusercontent.com/FurqanSoftware/node-whois/master/servers.json";

fn get_server_path() -> std::result::Result<String, std::io::Error> {
    let path = env::current_dir()?;
    let sub_path = path.to_str().ok_or("");
    Ok(sub_path.unwrap().to_string() + "/" + SERVER_PATH)
}

pub fn get_server_file_path() -> String {
    get_server_path().unwrap() + "/" +  SERVER_FILE
}

fn server_file_exists() -> bool {
    Path::new(&get_server_file_path()).exists()
}

fn create_config_path() {
    fs::create_dir_all(&get_server_path().unwrap()).expect("failed creating the config path");
}

// got this mostly from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
async fn download_server_file()-> Result<()> {
    create_config_path();
    
    let response = reqwest::get(SERVER_FILE_SOURCE).await?;

    let mut content = Cursor::new(response.bytes().await?);
    let mut file = File::create(get_server_file_path())?;

    copy(&mut content, &mut file);
    Ok(())
}

pub(crate) async fn get_server_file() -> bool {
    if !server_file_exists() {
        return match download_server_file().await {
            Ok(()) => true,
            _ => false
        };
    }

    server_file_exists()
}