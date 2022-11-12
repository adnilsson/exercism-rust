use std::collections::HashMap;

fn word_count<'a>(words: &[&'a str]) -> HashMap<&'a str, u32> {
    let mut counts = HashMap::with_capacity(words.len());
    for word in words {
        counts.entry(*word).and_modify(|e| *e += 1).or_insert(1);
    }
    counts
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // Smiple heuristic: it's impossible to build 'note' from 'magazine'
    // if the note has more words than the magazine.
    if note.len() > magazine.len() {
        return false;
    }

    let mut cutouts = word_count(magazine);
    for word in note {
        match cutouts.get_mut(word) {
            Some(0) => return false,
            Some(cutout_count) => {
                *cutout_count -= 1;
            }
            None => return false,
        }
    }

    true
}
