use std::{env, fs};
use crate::domain::result::WhoisResult;

const BASE_PATH: &str = "data/domains";

pub(crate) fn write_domain(record: &WhoisResult) -> std::io::Result<()> {
    std::fs::write(
        get_data_path(record.domain.clone()).unwrap(),
        serde_json::to_string_pretty(&record).unwrap(),
    )
}

fn get_data_path(name: String) -> std::result::Result<String, std::io::Error> {
    let path = env::current_dir()?;
    let base_path = path.to_str().ok_or("could not unwind base path");

    fs::create_dir_all(base_path.unwrap().to_string() + "/" + BASE_PATH).expect("failed creating the domain path");

    Ok(base_path.unwrap().to_string() + "/" + BASE_PATH + "/" + &*name.to_string())
}