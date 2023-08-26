use std::collections::HashSet;

mod anagram {
    use std::collections::{HashMap, HashSet};
    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
        // let word_hash: HashSet<char> = word.to_lowercase().chars().collect();
        let make_char_map = |mut acc: HashMap<char, i32>, ch| {
            let count = acc.entry(ch).or_insert(0);
            *count += 1;
            acc
        };

        let word_lower = word.to_lowercase();

        let word_map = word_lower.chars().fold(HashMap::new(), make_char_map);

        possible_anagrams
            .iter()
            .filter(|item| {
                let item_lower = item.to_lowercase();
                if (word_lower == item_lower) {
                    return false;
                }

                if (word.len() != item.len()) {
                    return false;
                }

                let item_map = item_lower.chars().fold(HashMap::new(), make_char_map);

                item.to_lowercase()
                    .chars()
                    .all(|ch| word_map.get(&ch).eq(&item_map.get(&ch)))
                // let intersect_hash = item_hash.intersection(&word_hash).clone().collect::<HashSet<&char>>();

                // intersect_hash.len() == word_hash.len()
            })
            .map(|&item| item)
            .collect()
    }
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagram::anagrams_for(word, inputs);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}
#[test]
fn test_no_matches() {
    let word = "diaper";
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_detect_simple_anagram() {
    let word = "ant";
    let inputs = ["tan", "stand", "at"];
    let outputs = vec!["tan"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_does_not_confuse_different_duplicates() {
    let word = "galea";
    let inputs = ["eagle"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_eliminate_anagram_subsets() {
    let word = "good";
    let inputs = ["dog", "goody"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_detect_anagram() {
    let word = "listen";
    let inputs = ["enlists", "google", "inlets", "banana"];
    let outputs = vec!["inlets"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_multiple_anagrams() {
    let word = "allergy";
    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    let outputs = vec!["gallery", "regally", "largely"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_case_insensitive_anagrams() {
    let word = "Orchestra";
    let inputs = ["cashregister", "Carthorse", "radishes"];
    let outputs = vec!["Carthorse"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_unicode_anagrams() {
    let word = "ΑΒΓ";
    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    let outputs = vec!["ΒΓΑ", "γβα"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_misleading_unicode_anagrams() {
    // Despite what a human might think these words contain different letters, the input uses Greek
    // A and B while the list of potential anagrams uses Latin A and B.
    let word = "ΑΒΓ";
    let inputs = ["ABΓ"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_does_not_detect_a_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["banana"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["bAnana"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "ΑΒΓ";
    let inputs = ["ΑΒγ"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_same_bytes_different_chars() {
    let word = "a⬂"; // 61 E2 AC 82
    let inputs = ["€a"]; // E2 82 AC 61
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
#[ignore]
fn test_different_words_but_same_ascii_sum() {
    let word = "bc";
    let inputs = ["ad"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
