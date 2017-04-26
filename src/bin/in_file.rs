extern crate word_counter;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
  let locs: Vec<String> = args().skip(1).collect();
  if locs.is_empty() {
    println!("Please specify a path or multiple paths");
    return;
  }
  let mut word_counts: Vec<HashMap<String, usize>> = Vec::new();
  for loc in locs {
    let mut f = match File::open(&loc) {
      Ok(f) => f,
      Err(e) => {
        println!("Could not open {}: {}", loc, e);
        return;
      }
    };
    let mut content = String::new();
    if let Err(e) = f.read_to_string(&mut content) {
      println!("Could not read {}: {}", loc, e);
      return;
    }
    word_counts.push(word_counter::count_words(&content));
  }
  let mut totals: Vec<(String, usize)> = total(word_counts).into_iter().collect();
  totals.sort_by_key(|&(_, v)| v);
  for (word, count) in totals {
    println!("{} => {}", word, count);
  }
}

fn total(maps: Vec<HashMap<String, usize>>) -> HashMap<String, usize> {
  let mut totals = HashMap::new();
  for map in maps {
    for (key, value) in map {
      *totals.entry(key).or_insert(0) += value;
    }
  }
  totals
}
