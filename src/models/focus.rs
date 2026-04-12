#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Field {
    Url,
    Method,
    Body,
    Response,
    None,
}

impl Field {
    pub const ALL: &[Field] = &[Field::Url, Field::Method, Field::Body, Field::Response];

    pub fn next(self) -> Field {
        match self {
            Field::Url => Field::Method,
            Field::Method => Field::Body,
            Field::Body => Field::Response,
            Field::Response => Field::Url,
            _ => Field::None,
        }
    }

    pub fn previous(self) -> Field {
        match self {
            Field::Url => Field::Response,
            Field::Method => Field::Url,
            Field::Body => Field::Method,
            Field::Response => Field::Body,
            _ => Field::None,
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
        Self {
            current: Field::Url,
        }
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
