use std::{collections::HashSet, fs, io};
use colored::Colorize;
use scraper::{Html, Selector};

pub struct AccountData {
    follower_html: String,
    following_html: String
}

impl AccountData {
    pub fn new(follower_file: String, following_file: String) -> Self {
        Self {
            follower_html: follower_file,
            following_html: following_file
        }
    }

    pub fn parse_html(file_path: String) -> Result<HashSet<(String, String)>, io::Error> {
        let html = fs::read_to_string(file_path)?;
        let fragment = Html::parse_fragment(&html);
        let selector = Selector::parse("a").unwrap();

        Ok(fragment
            .select(&selector)
            .filter_map(|element| {
                assert_eq!("a", element.value().name());

                let username = element.text().next().map(|text| text.to_owned());
                let href = element.value().attr("href").map(|href| href.to_owned());

                username.and_then(|username| href.map(|href| (username, href)))
            })
            .collect())
    }

    pub fn collect_data(&self) -> (HashSet<(String, String)>, HashSet<(String, String)>) {
        let followers = AccountData::parse_html(self.follower_html.clone()).unwrap_or_else(|err| {
            eprintln!("Failed to read the file containing your followers: {}", err);
            std::process::exit(1);
        });

        let following = AccountData::parse_html(self.following_html.clone()).unwrap_or_else(|err| {
            eprintln!("Failed to read the file containing the accounts your are following: {}", err);
            std::process::exit(1);
        });

        (followers, following)
    }

    pub fn fetch_data<Data>(&self, mut result: Data) -> u32
    where Data: FnMut(&str) {
        let (followers, following) = self.collect_data();
        let users: HashSet<_> = following.difference(&followers).collect();
        let mut counter = 1;

        if users.is_empty() {
            println!("{}", "[*] Nothing to see here.. (^._.^)~".green());
        } else {
            println!("{}", "[*] These accounts do not follow you back:".blue());

            for (user, link) in users {
                result(&format!("[!] {} - {}", user, link));
                counter += 1;
            }
        }

        counter
    }
}