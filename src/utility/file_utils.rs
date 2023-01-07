use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_line_by_line(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut txs: Vec<String> = vec![];
    for line in reader.lines() {
        txs.push(line?);
    }

    Ok(txs)
}
