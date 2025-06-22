use std::{fs::File, io::{self, Write}};

pub fn filter_from_entry(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split(" - ").collect();

    if parts.len() == 2 {
        let username = parts[0].trim_start_matches("[!] ").trim();
        let url = parts[1].trim();

        return Some((username.to_string(), url.to_string()));
    }

    None
}

pub fn write_html(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}