// Undo.rs - Utilities for undoing, redoing and storing events
use crate::Position;

// Event enum to store the types of events that occur
#[derive(Clone, Copy, Debug)]
pub enum Event {
    // Cursor & Offset, Grapheme, Character, Previous x
    Insert(Position, usize, char, usize),
    Delete(Position, usize, char, usize),
}

// A struct for holding all the events taken by the user
#[derive(Debug)]
pub struct EventStack {
    history: Vec<Event>, // For storing the history of events
}

// Methods for the EventStack
impl EventStack {
    pub fn new() -> Self {
        // Initialise an Event stack
        Self { history: vec![] }
    }
    pub fn push(&mut self, event: Event) {
        // Add an event to the event stack
        self.history.push(event);
    }
    pub fn pop(&mut self) -> Option<Event> {
        // Take an event off the event stack
        self.history.pop()
    }
    pub fn empty(&mut self) {
        // Clear the event stack
        self.history.clear();
    }
    pub fn undo(&mut self) -> Option<Event> {
        // Perform an undo operation
        if let Some(element) = self.pop() {
            Some(EventStack::reverse(element))
        } else {
            None
        }
    }
    pub fn redo(&mut self) {
        // Perform a redo operation
    }
    pub fn reverse(event: Event) -> Event {
        // Reverse an event
        match event {
            Event::Insert(pos, graphemes, ch, px) => Event::Delete(pos, graphemes, ch, px),
            Event::Delete(pos, graphemes, ch, px) => Event::Insert(pos, graphemes, ch, px),
        }
    }
}