use std::{env, fs::{self, File}, io::{self, Write}};

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

pub struct Sources {
    pub follower: String,
    pub following: String
}

pub fn locate_sources() -> Result<Sources, io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Ok(Sources { 
            follower: "followers_1.html".to_string(), 
            following: "following.html".to_string() 
        })
    }

    let dir = &args[1];
    let metadata = fs::metadata(dir)?;

    if metadata.is_dir() {
        Ok(Sources { 
            follower: format!("{}/connections/followers_and_following/followers_1.html", dir), 
            following: format!("{}/connections/followers_and_following/following.html", dir) 
        })
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("{} is not a directory", dir)
        ))
    }
}