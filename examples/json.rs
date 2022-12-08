use parcomb::parser::{lst_sep_empt, ParseResult, Parser};
use parcomb::string_parser::{lit, lit_sp, reg, spaces};

use std::collections::HashMap;

/// https://www.json.org/json-en.html
#[derive(Debug, PartialEq)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
}

type JsonResult<'a> = ParseResult<&'a str, JsonValue, String>;

fn null_p() -> impl Parser<str, JsonValue, String> {
    lit("null").map(|_| JsonValue::Null)
}

fn bool_p() -> impl Parser<str, JsonValue, String> {
    let true_p = lit("true").map(|_| JsonValue::Bool(true));
    let false_p = lit("false").map(|_| JsonValue::Bool(false));

    true_p.or(false_p)
}

fn main() {
    let input = " {  false,true, true, false } ";

    let par = lit_sp("{")
        .and_r(lst_sep_empt(bool_p(), lit_sp(",")))
        .and_l(lit_sp("}"));

    println!("{:#?}", par.parse(input));
}
