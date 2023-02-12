use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn grapheme_sort(word: &str) -> String {
    let mut grapheme_clusters: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
    grapheme_clusters.sort_unstable();

    grapheme_clusters.join("")
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowered_word = word.to_lowercase();
    let target_word = grapheme_sort(&lowered_word);

    let lowered_candidate_words: Vec<String> =
        possible_anagrams.iter().map(|w| w.to_lowercase()).collect();
    let lowered_and_sorted = lowered_candidate_words.iter().map(|x| grapheme_sort(&x));

    lowered_and_sorted
        .enumerate()
        .fold(HashSet::new(), |mut acc, (i, candidate)| {
            if candidate == target_word && lowered_word != lowered_candidate_words[i] {
                acc.insert(possible_anagrams[i]);
            }
            acc
        })
}
