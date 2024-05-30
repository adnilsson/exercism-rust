use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn grapheme_sort(word: &str) -> Vec<&str> {
    let mut grapheme_clusters: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    grapheme_clusters.sort_unstable();
    grapheme_clusters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowered_word = word.to_lowercase();
    let target_word = grapheme_sort(&lowered_word);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let lowered_candidate = candidate.to_lowercase();
            lowered_candidate != lowered_word && grapheme_sort(&lowered_candidate) == target_word
        })
        .copied()
        .collect()
}
