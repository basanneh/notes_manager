
struct Note {
    id: u32,
    text: String,
}

struct Notes {
    items: Vec<Note>,
}

impl Notes {
    fn add(&mut self, text: &str);
    fn list(&self);
    fn delete(&mut self, id: u32) -> bool;
    fn get(&self, id: u32) -> Option<&Note>;
}

fn main() {}
