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

fn null_p<'a>(input: &'a str) -> JsonResult<'a> {
    lit("null").map(|_| JsonValue::Null).parse(input)
}

fn bool_p<'a>(input: &'a str) -> JsonResult<'a> {
    let true_p = lit("true").map(|_| JsonValue::Bool(true));
    let false_p = lit("false").map(|_| JsonValue::Bool(false));

    true_p.or(false_p).parse(input)
}

fn number_p<'a>(input: &'a str) -> JsonResult<'a> {
    let sign_pat = r"-";
    let int_pat = r"0|([123456789]\d*)";
    let frag_pat = r"\.\d+";
    let exp_pat = r"[eE][\+-]\d+";

    let ptn = format!("({sign_pat})?({int_pat})({frag_pat})?({exp_pat})?");
    let par = reg(&ptn).map(|s| JsonValue::Number(f64::from_str(&s).unwrap()));

    par.parse(input)
}

fn string_p<'a>(input: &'a str) -> JsonResult<'a> {
    let str_char_ptn = r#"[^\\"]"#;
    let esc_char_ptn = r#"\\[\\/"bfnrt]"#;
    let utf16_char_ptn = r#"\\u[a-fA-F0-9]{4}"#; // todo: should conver to u16

    let json_str_ptn = format!(r#""(({str_char_ptn})|({esc_char_ptn})|({utf16_char_ptn}))*""#);

    let par = reg(&json_str_ptn).map(|s| JsonValue::String(s));

    par.parse(input)
}

fn array_p<'a>(input: &'a str) -> JsonResult<'a> {
    let par = lit_sp("[")
        .and_r(lst_sep_empt(value_p, lit_sp(",")))
        .and_l(lit_sp("]"))
        .map(|arr| JsonValue::Array(arr));

    par.parse(input)
}

fn object_p<'a>(input: &'a str) -> JsonResult<'a> {
    todo!()
}

fn value_p<'a>(input: &'a str) -> JsonResult<'a> {
    let par = null_p
        .or(bool_p)
        .or(number_p)
        .or(string_p)
        .or(array_p)
        .or(object_p);

    par.parse(input)
}

fn main() {
    let input = r#" [  false,99, "\u2192the", true, true, 32, 1.321, "aaa", null ] "#;

    println!("{:#?}", value_p.parse(input));
}
