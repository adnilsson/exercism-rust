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

    let cutouts = word_count(magazine);
    let required_words = word_count(note);

    for (note_word, note_count) in required_words {
        match cutouts.get(note_word) {
            Some(cutout_count) => {
                if *cutout_count < note_count {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
