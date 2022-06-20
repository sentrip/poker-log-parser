mod pklp;

// TODO: Command line utility for using pklp

fn test_parser_files(dir: &str) -> (usize, usize, f64) {
    let mut files = Vec::new();
    let mut paths = Vec::new();
    let mut counts = Vec::new();
    for path in std::fs::read_dir(dir).unwrap() {
        paths.push(path.unwrap().path().to_str().unwrap().to_owned());
        files.push(std::fs::read_to_string(&paths.last().as_ref().unwrap()).unwrap());
        counts.push(0);
    }
    
    let now = std::time::Instant::now();

    for i in 0..files.len() {
        let hands = pklp::parse_string(&files[i]);
        counts[i] = hands.len();
    }

    let elapsed = now.elapsed();

    let parsed = counts.iter().sum();
    let mut total = 0;
    for f in &files { total += f.matches("PokerStars Hand").count(); }

    (parsed, total, (elapsed.as_micros() as u64) as f64 / 1000000.0)
}


fn main() {
    // let data = std::fs::read_to_string("data/example/pokerstars_example.txt").unwrap();
    // let hands = pklp::parse_string(&data);
    // let j = pklp::to_json(&hands).unwrap();
    // println!("Json size: {}", j.len());
    
    let (parsed, total, elapsed) = test_parser_files("data/PokerStars/");
    println!("Parsed {}/{} in {} secs ({:.2} per/sec)", parsed, total, elapsed, parsed as f64 / elapsed);
}
