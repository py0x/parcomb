use parcomb::parser::{lst_sep_empt, ParseResult, Parser};
use parcomb::string_parser::{lit, lit_sp, reg, spaces};

use std::collections::HashMap;
use std::str::FromStr;

/// https://www.json.org/json-en.html
#[derive(Debug, PartialEq)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
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

fn number_p() -> impl Parser<str, JsonValue, String> {
    let sign_pat = r"-";
    let int_pat = r"0|([123456789]\d*)";
    let frag_pat = r"\.\d+";
    let exp_pat = r"[eE][\+-]\d+";

    let ptn = format!("({sign_pat})?({int_pat})({frag_pat})?({exp_pat})?");
    let par = reg(&ptn).map(|s| JsonValue::Number(f64::from_str(&s).unwrap()));

    return par;
}

fn main() {
    let input = " {  false,99,true, true, 32, 1.321 } ";

    let par = lit_sp("{")
        .and_r(lst_sep_empt(bool_p().or(number_p()), lit_sp(",")))
        .and_l(lit_sp("}"));

    println!("{:#?}", par.parse(input));
}
