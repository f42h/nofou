mod core;
use core::{data::AccountData, generator::HtmlOutput};

use colored::Colorize;

fn main() {
    let mut output_gen = HtmlOutput::new();
    let data = AccountData::new("followers.html".to_string(), "following.html".to_string());

    let count = data.fetch_data(|account| {
        output_gen.add_account(account);
        println!("{}", account.red());
    });

    output_gen.close();

    println!("{}", format!("[*] Total of {} accounts were identified", count).blue());
}
