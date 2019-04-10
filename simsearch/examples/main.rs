use std::fs::File;
use std::io;
use std::io::Read;

use json;
use simsearch::SimSearch;

fn main() -> io::Result<()> {
    let mut engine = SimSearch::new();

    let mut file = File::open("./books.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let j = json::parse(&content).unwrap();

    for book in j.members() {
        let title = book["title"].as_str().unwrap();
        engine.insert(
            title.to_owned(),
            &title.split_whitespace().collect::<Vec<&str>>(),
        );
    }

    loop {
        let mut pattern = String::new();
        io::stdin()
            .read_line(&mut pattern)
            .expect("failed to read from stdin");

        let pattern = pattern.replace("\r\n", "");
        let tokens: Vec<&str> = pattern.split_whitespace().collect();

        println!("pattern: {:?}", &pattern);
        println!("result: {:?}", engine.search(&tokens));
    }
}
