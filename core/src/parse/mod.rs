use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedClass {
    pub raw: String,
    pub variants: Vec<Variant>,
    pub family: Family,
    pub utility: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Fluent,
    Material,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Hover,
    Focus,
    Active,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    InvalidVariant(String),
    MissingPrefix(String),
    MissingUtility(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty class name"),
            Self::InvalidVariant(value) => write!(f, "unsupported variant `{value}`"),
            Self::MissingPrefix(value) => write!(f, "missing motif prefix in `{value}`"),
            Self::MissingUtility(value) => write!(f, "missing utility in `{value}`"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_class_name(input: &str) -> Result<ParsedClass, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }

    let mut segments: Vec<&str> = input.split(':').collect();
    let base = segments.pop().ok_or(ParseError::Empty)?;
    let variants = segments
        .into_iter()
        .map(parse_variant)
        .collect::<Result<Vec<_>, _>>()?;

    let (family, body) = if let Some(rest) = base.strip_prefix("f-") {
        (Family::Fluent, rest)
    } else if let Some(rest) = base.strip_prefix("m-") {
        (Family::Material, rest)
    } else if let Some(rest) = base.strip_prefix("ui-") {
        (Family::Universal, rest)
    } else {
        return Err(ParseError::MissingPrefix(input.to_string()));
    };

    if body.is_empty() {
        return Err(ParseError::MissingUtility(input.to_string()));
    }

    let mut body_segments = body.splitn(2, '-');
    let utility = body_segments.next().unwrap_or_default();
    if utility.is_empty() {
        return Err(ParseError::MissingUtility(input.to_string()));
    }

    let value = body_segments.next().map(str::to_string);

    Ok(ParsedClass {
        raw: input.to_string(),
        variants,
        family,
        utility: utility.to_string(),
        value,
    })
}

fn parse_variant(input: &str) -> Result<Variant, ParseError> {
    match input {
        "hover" => Ok(Variant::Hover),
        "focus" => Ok(Variant::Focus),
        "active" => Ok(Variant::Active),
        "dark" => Ok(Variant::Dark),
        _ => Err(ParseError::InvalidVariant(input.to_string())),
    }
}
