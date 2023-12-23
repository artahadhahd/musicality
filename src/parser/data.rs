#[derive(Debug)]
pub enum NoteModifier {
    Flat,
    Sharp,
    None,
}

#[derive(Debug)]
#[rustfmt::skip]
pub enum NoteName {
    A, B, C, D, E, F, G,
}

#[derive(Debug)]
pub struct Note {
    pub note: NoteName,
    pub modifier: NoteModifier,
}

#[derive(Debug)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: f32,
}

impl Chord {
    #[allow(dead_code)]
    pub fn is_note(&self) -> bool {
        self.notes.len() == 1
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: f32,
}
