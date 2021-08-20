use std::collections::HashMap;

fn main() {
  let v: Vec<String> = vec!["ab".to_string(), "ba".to_string(), "tea".to_string(), "eat".to_string(), "ball".to_string(), "labl".to_string(), "fee".to_string(), "rise".to_string(), "dave".to_string(), "vade".to_string(), "some".to_string(), "mose".to_string(), "redf".to_string(), "osem".to_string()];

  group_anagrams(v);
}

fn group_anagrams(strs: Vec<String>) {
  let mut hash_table = HashMap::new();

  for item in &strs {
    let mut split_str: Vec<String> = item.split("").filter(|&x| !x.is_empty()).map(|s| s.to_string()).collect();
    split_str.sort();

    let key = split_str.join("");
    
    let v = hash_table.entry(key).or_insert(Vec::new());
    v.push(item);
  }

  let mut anograms: Vec<&Vec<&String>> = Vec::new();
  for (_, v) in &hash_table {
    anograms.push(v);
  }

  println!("Anagrams: {:?}", anograms);
}