use std::{io::Write, path::PathBuf, process::{Command, Stdio}, str::FromStr};

fn main() {
    let os_username = users::get_current_username().unwrap();
    let username = os_username.to_str().unwrap();
    let mut product_path = PathBuf::from_str(
        format!("/Users/{}", username).as_str()
    ).unwrap();
    product_path.push("Library/Containers/io.playcover.PlayCover/Applications/com.tong.lcgame.app/ProductName");

    if product_path.exists() {
        let mut cmd_child = Command::new("zsh")
            .stdin(Stdio::piped()).spawn().expect("Failed to launch zsh");
        let mut stdin = cmd_child.stdin.take().expect("Failed to open input pipe");

        let string_product_path = product_path.as_os_str().to_str().unwrap();
        stdin.write_all(string_product_path.as_bytes()).expect("Failed to write into stdin");
    } else {
        println!("PlayCover not installed");
    }
}
