
#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Field {
    Url,
    Method,
    Headers,
    Body,
    Response,
}

impl Field {
    pub const ALL: &[Field] = &[
        Field::Url,
        Field::Method,
        Field::Headers,
        Field::Body,
        Field::Response
    ];

    pub fn next(self) -> Field {
        match self {
            Field::Url => Field::Method,
            Field::Method => Field::Headers,
            Field::Headers => Field::Body,
            Field::Body => Field::Response,
            Field::Response => Field::Url
        }
    }

    pub fn previous(self) -> Field {
        match self {
            Field::Url => Field::Response,
            Field::Method => Field::Url,
            Field::Headers => Field::Method,
            Field::Body => Field::Headers,
            Field::Response => Field::Body
        }
    }

    pub fn from_index(index: usize) -> Option<Field> {
        Self::ALL.get(index).copied()
    }
    
    pub fn index(self) -> usize {
        self as usize
    }
}


pub struct FocusManager {
    current: Field,
}


impl FocusManager {
    pub fn new() -> Self {
        Self { current: Field::Url }
    }

    pub fn current(&self) -> Field {
        self.current
    }

    pub fn is_focused(&self, field: Field) -> bool {
        self.current == field
    }

    pub fn next(&mut self) {
        self.current = self.current.next();
    }

    pub fn previous(&mut self) {
        self.current = self.current.previous();
    }
}
