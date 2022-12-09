use super::parser::{ParseResult, Parser};
use super::string_parser::*;
use std::str::FromStr;

#[test]
fn test_reg_parse_integer() {
    let ptn = r"0|([123456789]\d*)";
    let par = reg(ptn).map(|s| i64::from_str(&s).unwrap());

    // single digit: 0
    {
        let inp = "0";
        let res = par.parse(inp);
        assert_eq!(Ok((0, "")), res);
    }

    // single digit: 8
    {
        let inp = "8";
        let res = par.parse(inp);
        assert_eq!(Ok((8, "")), res);
    }

    // multiple digits
    {
        let inp = "1234590";
        let res = par.parse(inp);
        assert_eq!(Ok((1234590, "")), res);
    }

    // zero prefixed
    {
        let inp = "01";
        let res = par.parse(inp);
        assert_eq!(Ok((0, "1")), res);
    }
}

#[test]
fn test_reg_parse_float() {
    let int_pat = r"0|([123456789]\d*)";
    let frag_pat = r"\.\d+";
    let ptn = format!("({int_pat})({frag_pat})?");
    let par = reg(&ptn).map(|s| f64::from_str(&s).unwrap());

    // happy path: 1
    {
        let inp = "0.1415";
        let res = par.parse(inp);
        assert_eq!(Ok((0.1415, "")), res);
    }

    // happy path: 2
    {
        let inp = "23.012";
        let res = par.parse(inp);
        assert_eq!(Ok((23.012, "")), res);
    }

    // without the fractional part
    {
        let inp = "1492";
        let res = par.parse(inp);
        assert_eq!(Ok((1492.0, "")), res);
    }
}

#[test]
fn test_reg_parse_number() {
    let sign_pat = r"-";
    let int_pat = r"0|([123456789]\d*)";
    let frag_pat = r"\.\d+";
    let exp_pat = r"[eE][\+-]\d+";

    let ptn = format!("({sign_pat})?({int_pat})({frag_pat})?({exp_pat})?");
    let par = reg(&ptn).map(|s| f64::from_str(&s).unwrap());

    // happy path: integer
    {
        let inp = "123";
        let res = par.parse(inp);
        assert_eq!(Ok((123.0, "")), res);
    }

    // happy path: negative float
    {
        let inp = "-0.1415";
        let res = par.parse(inp);
        assert_eq!(Ok((-0.1415, "")), res);
    }

    // happy path: exp
    {
        let inp = "0.1415E-3";
        let res = par.parse(inp);
        assert_eq!(Ok((0.0001415, "")), res);
    }

    // happy path: exp
    {
        let inp = "3.1415E+2";
        let res = par.parse(inp);
        assert_eq!(Ok((314.15, "")), res);
    }
}