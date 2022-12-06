use crate::parser::{Parser, ParseResult};

pub struct LiteralParser {
    literal: String
}

impl Parser<str, String, String> for LiteralParser {
    fn parse<'a>(&self, input: &'a str) -> ParseResult<&'a str, String, String> {
        if input.starts_with(&self.literal) {
            let remain = &input[self.literal.len()..];
            let res = (self.literal.clone(), remain);

            return Ok(res);
        }

        return Err("unmatch".to_string());
    }
}

pub fn lit(s: &str) -> LiteralParser {
    LiteralParser { literal: s.to_string() }
}

// Regex parser