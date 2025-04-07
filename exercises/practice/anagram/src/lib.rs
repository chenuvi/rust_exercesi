use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let sorted = sort(&lower);
    possible_anagrams
        .iter()
        .filter(|input| {
            let input_lower = input.to_lowercase();
            lower != input_lower && sorted == sort(&input_lower)
        })
        .cloned()
        .collect()
}

fn sort(word: &str) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted.into_iter().collect()
}
