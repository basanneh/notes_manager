use notes_manager::notes::Notes;

#[test]
fn add_and_get_note() {
    let mut notes = Notes::new();
    notes.add("Buy new car");
    let n = notes.get(1).unwrap();
    assert_eq!(n.text, "Buy new car");
}

#[test]
fn delete_note() {
    let mut notes = Notes::new();
    notes.add("Buy insurance");
    notes.add("Register the car");
    assert!(notes.delete(1));
    assert!(notes.get(1).is_none());
    assert!(notes.get(2).is_some());
    
}