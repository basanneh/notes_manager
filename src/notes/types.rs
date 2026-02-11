#[derive(Debug, Clone)]
pub struct Note {
    pub id: u32,
    pub text: String,
}

#[derive(Debug)]
pub struct Notes {
    pub items: Vec<Note>,
    pub next_id: u32,
}

impl Notes {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add(&mut self, text: &str) {
        let note = Note {
            id: self.next_id,
            text: text.to_string(),
        };
        self.next_id += 1;
        self.items.push(note);
    }

    pub fn list(&self) -> &[Note] {
        &self.items
    }

    pub fn delete(&mut self, id: u32) -> bool {
        let before = self.items.len();
        self.items.retain(|n| n.id != id);
        self.items.len() != before
    }
    
    pub fn get(&self, id: u32) -> Option<&Note> {
        self.items.iter().find(|n| n.id == id)
    }
}