// src/models/http.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    pub const ALL: &[HttpMethod] = &[
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Delete,
        HttpMethod::Patch,
        HttpMethod::Head,
        HttpMethod::Options,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }

    pub fn next(self) -> HttpMethod {
        match self {
            HttpMethod::Get => HttpMethod::Post,
            HttpMethod::Post => HttpMethod::Put,
            HttpMethod::Put => HttpMethod::Delete,
            HttpMethod::Delete => HttpMethod::Patch,
            HttpMethod::Patch => HttpMethod::Head,
            HttpMethod::Head => HttpMethod::Options,
            HttpMethod::Options => HttpMethod::Get,
        }
    }

    pub fn previous(self) -> HttpMethod {
        match self {
            HttpMethod::Get => HttpMethod::Options,
            HttpMethod::Post => HttpMethod::Get,
            HttpMethod::Put => HttpMethod::Post,
            HttpMethod::Delete => HttpMethod::Put,
            HttpMethod::Patch => HttpMethod::Delete,
            HttpMethod::Head => HttpMethod::Patch,
            HttpMethod::Options => HttpMethod::Head,
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}