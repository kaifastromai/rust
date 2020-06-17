use std::char;
struct Note {
    value: NoteValue,
    accidental: Accidental,
}
impl Note {
    fn new(self: &mut Self, value: NoteValue, accidental: Accidental) {
        self.value = value;
        self.accidental = accidental;
    }

    fn parse_note_name() -> Option<Note> {
        Some(Note {
            value: NoteValue::C,
            accidental: Accidental::Flat,
        })
    }

    fn is_valid_note(v: Vec<char>) -> bool {
        let c = v[0];
        let l = c == 'a';

        if c == 'A' || c == 'B' || c == 'C' || c == 'D' || c == 'E' || c == 'F' || c == 'G' {
            if v[1] == 'b' || v[1] == '#' {
                return true;
            }
            return false;
        } 
        return false
    }
}
struct Scale {}
enum ScaleMode {
    Major,
    Minor,
}
enum NoteValue {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
    C,
}
enum Accidental {
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}
