mod core;
use core::{data::AccountData, generator::HtmlOutput};

use colored::Colorize;

use crate::core::utils::locate_sources;

fn main() {
    let srcs = match locate_sources() {
        Ok(srcs) => srcs,
        Err(err) => {
            eprintln!("Error: Failed to locate HTML files: {}", err);
            return;
        }
    };


    let mut output_gen = HtmlOutput::new();
    let data = AccountData::new(srcs.follower, srcs.following);

    let count = data.fetch_data(|account| {
        output_gen.add_account(account);
        println!("{}", account.red());
    });

    output_gen.close();

    println!("{}", format!("[*] Total of {} accounts were identified", count).blue());
}
