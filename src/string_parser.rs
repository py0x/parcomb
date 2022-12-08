use crate::parser::{ParseResult, Parser};
use regex::Regex;
pub struct LiteralParser {
    literal: String,
}

impl Parser<str, String, String> for LiteralParser {
    fn parse<'a>(&self, input: &'a str) -> ParseResult<&'a str, String, String> {
        if input.starts_with(&self.literal) {
            let remain = &input[self.literal.len()..];
            let res = (self.literal.clone(), remain);

            return Ok(res);
        }

        return Err(format!("LiteralParser: No Match"));
    }
}

pub struct RegexParser {
    re: Regex,
}

/// # Example:
/// ```
/// use parcomb::string_parser::*;
/// use parcomb::parser::{Parser, ParseResult};
///
/// let par = reg(r"\d{2}\w+");
/// let inp = "19abcd$$";
/// let res = par.parse(inp).unwrap();
/// assert_eq!(("19abcd".to_string(), "$$"), res);
///
/// let inp2 = "ccc19abcd$$";
/// let res2 = par.parse(inp2);
/// assert!(res2.is_err());
/// ```
impl Parser<str, String, String> for RegexParser {
    fn parse<'a>(&self, input: &'a str) -> ParseResult<&'a str, String, String> {
        match self.re.find(input) {
            None => Err(format!("RegexParser: No Match")),
            Some(mat) => {
                let out = mat.as_str().to_string();
                let remain_i = &input[(mat.end()..)];
                Ok((out, remain_i))
            }
        }
    }
}

pub fn lit(s: &str) -> LiteralParser {
    LiteralParser {
        literal: s.to_string(),
    }
}

pub fn reg(re: &str) -> RegexParser {
    let re_pattern = format!("^{}", re);
    let re = Regex::new(&re_pattern).unwrap();

    RegexParser { re }
}

pub fn spaces() -> RegexParser {
    reg(r"(\s)*")
}

pub fn lit_sp(s: &str) -> impl Parser<str, String, String> {
    spaces().and_r(lit(s)).and_l(spaces())
}
