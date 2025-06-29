# NoFoU - Offline tool to check who's not following you back on Instagram

### Requirements

- [Rust](https://rustup.rs/) installation

### Getting Started

- Download your account data from [Instagram](https://accountscenter.instagram.com/info_and_permissions/dyi/) (It will take a while for Instagram to generate your stuff. It'll be send to you via E-Mail)
1. Select the `Download or transfer information` button.
2. Choose the `Some of your information` option.
3. Scroll down to locate and select the `Followers and Following` option.
4. Proceed by selecting `Next`.
5. Opt for `Download on device`.
6. Finish by selecting `Create files`.
- Unzip the downloaded data and search for `followers_1.html` and `following.html`: `connections/` -> `followers_and_following/`
- Copy followers.html and following.html and paste it to the nofou project directory
- For console output execute the following command in the console
```
cargo run .
```
- You can also specify the direcory as a parameter for automated search
```
cargo run <directory_path>
```
- If only the HTML output is required, just execute `start.bat` with a double click

#### This will run the console for you and generate a result file that can be opened within the preferred browser. Only ensure that `followers.html` and `following.html` are in the `same directory` as the `start.bat`.