use std::fs;
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("arg start");
    // let args = std::env::args().collect::<Vec<String>>();

    // let args: Vec<String> = std::env::args().collect();
    // let url = &args[1];
    // println!("args: {:?}", args);
    // println!("url: {}", url);
    // if let [_path, url, output, ..] = args.as_slice() {
    //     println!("url: {}, output: {}", url, output);
    // } else {
    //     eprintln!("参数缺失");
    // }

    for arg in std::env::args() {
        println!("{}", arg);
    }
    println!("arg end");

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
