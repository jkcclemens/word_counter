use std::collections::HashMap;

pub fn count_words(input: &str) -> HashMap<String, usize> {
  let mut map = HashMap::new();
  for word in input.split_whitespace() {
    let word: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
    *map.entry(word.to_lowercase()).or_insert(0) += 1;
  }
  map
}
