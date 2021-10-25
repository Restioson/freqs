use std::cmp::Reverse;
use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::io::BufWriter;
use csv::Writer;
use itertools::Itertools;

fn main() {
    let dir = fs::read_dir(env::args().nth(1).expect("Pass the directory by the command line")).unwrap();
    let needles = env::args().skip(2);
    let mut words: HashMap<String, u64> = needles.map(|w| (w.to_lowercase(), 0)).collect();

    let mut csv = Writer::from_writer(BufWriter::new(File::create("freqs.csv").unwrap()));
    csv.write_record(&["filename", "word", "count"]).unwrap();

    for entry in dir.map(Result::unwrap).sorted_by_key(|e| e.path()) {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        let content = fs::read_to_string(&path).unwrap();

        for word in content.split_whitespace() {
            let word = word
                .replace(|c: char| c.is_ascii_punctuation(), "")
                .to_lowercase();
            words.entry(word).and_modify(|count| *count += 1);
        }

        for (word, count) in words.iter_mut().sorted_by_key(|(_w, c)| Reverse(**c)) {
            csv.write_record(&[&file_name as &str, &word, &count.to_string()]).unwrap();
            *count = 0;
        }
    }
}
