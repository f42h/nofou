use crate::core::utils::{filter_from_entry, write_html};

const BEGIN: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NoFo Results</title>
    <style>
        Body {
            background-color: rgb(26, 26, 26);
            font-family: "Lucida Console", "Courier New", monospace;
            color: #ffffff;
        }

        a {
            text-decoration: none;
            color: inherit;
        }

        a:hover {
            color: red;
        }

        a:hover:before {
            content: "Click to visit the profile of "
        }
    </style>
</head>
<body>
    <h1>These accounts do not follow you back:</h1>
    <ul>
"#;

const END: &str = r#"
    </ul>
</body>
</html>"#;

pub struct HtmlOutput {
    output: String
}

impl HtmlOutput {
    pub fn new() -> Self {
        let mut s = String::new();
        s.push_str(BEGIN);

        Self {
            output: s
        }
    }

    pub fn add_account(&mut self, entry: &str) {
        if let Some((username, href)) = filter_from_entry(entry) {
            self.output.push_str(&format!(r#"<li><a href="{}">{}</a></li>"#, href, username));
        } 
    }

    pub fn close(&mut self) {
        self.output.push_str(END);
        write_html("output.html", &self.output).unwrap();
    }
}