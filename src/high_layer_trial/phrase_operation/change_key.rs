use std::collections::{BTreeMap, BTreeSet};

use super::super::super::data::music_info::{Note, Phrase};

pub fn change_key(phrase: Phrase, key: f32) -> Phrase {
    let mut new_notes = BTreeMap::new();
    for (&start, note_set) in phrase.notes.iter() {
        let mut new_note_set = BTreeSet::new();
        for note in note_set.iter() {
            new_note_set.insert(Note {
                pitch: note.pitch.add_f32(key),
                duration: note.duration,
                start: note.start,
            });
        }
        new_notes.insert(start, new_note_set);
    }

    let new_length = phrase.length;

    Phrase {
        notes: new_notes,
        length: new_length,
    }
}