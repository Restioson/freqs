use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use csv::Writer;
use itertools::Itertools;

fn main() {
    let mut words: HashMap<String, u64> = HashMap::new();
    let mut csv = Writer::from_writer(BufWriter::new(File::create("freqs.csv").unwrap()));

    fs::read_dir(std::env::args().nth(1).expect("Pass the directory by the command line"))
        .unwrap()
        .map(|path| fs::read_to_string(path.unwrap().path()).unwrap())
        .flat_map(|str| str.split_whitespace().map(|s| s.to_owned()).collect::<Vec<_>>())
        .map(|word: String| word.replace(|c: char| c.is_ascii_punctuation(), ""))
        .for_each(|word| *words.entry(word).or_insert(0) += 1);

    csv.write_record(&["word", "count"]).unwrap();

    words
        .into_iter()
        .sorted_by_key(|(_w, c)| Reverse(*c))
        .map(|(w, c)| [w, c.to_string()])
        .for_each(|row| csv.write_record(&row).unwrap());
}
