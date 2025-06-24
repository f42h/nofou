# NoFoU - Offline tool to check who's not following you back on Instagram

### Requirements

- [Rust](https://rustup.rs/) installation

### Getting Started

- Download your account data from [Instagram](https://accountscenter.instagram.com/info_and_permissions/dyi/) (It will take a while for Instagram to generate your stuff, before it'll be send to you via E-Mail)
- Unzip the downloaded data and search for `followers.html` (or followers_1.html, if so rename followers_1.html to followers.html) and `following.html`: `connections/` -> `followers_and_following/`
- Copy followers.html and following.html and paste it to the nofou project directory
- For console output execute the following command in the console
```
cargo run .
```
- If only the HTML output is required, just execute `start.bat` with a double click

#### This will run the console for you and generate a result file that can be opened within the preferred browser. Only ensure that `followers.html` and `following.html` are in the `same directory` as the `start.bat`.