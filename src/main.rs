mod replace;

use replace::Replace;

fn main() -> std::io::Result<()> {
    let src = String::from("static/C++.txt");
    let dist = String::from("static/Rust.txt");
    let target = String::from("C++");
    let word = String::from("Rust");

    let mut replace = Replace::new(src, dist, target, word);
    replace.find_and_replace()?;
    Ok(())
}
