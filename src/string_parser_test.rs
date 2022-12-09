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
    let int_ptn = r"0|([123456789]\d*)";
    let frag_ptn = r"\.\d+";
    let ptn = format!("({int_ptn})({frag_ptn})?");
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
    let sign_ptn = r"-";
    let int_ptn = r"0|([123456789]\d*)";
    let frag_ptn = r"\.\d+";
    let exp_ptn = r"[eE][\+-]\d+";

    let ptn = format!("({sign_ptn})?({int_ptn})({frag_ptn})?({exp_ptn})?");
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

#[test]
fn test_reg_parse_json_string() {
    let str_char_ptn = r#"[^\\"]"#;
    let esc_char_ptn = r#"\\[\\/"bfnrt]"#;
    let utf16_char_ptn = r#"\\u[a-fA-F0-9]{4}"#;

    let json_str_ptn = format!(r#""(({str_char_ptn})|({esc_char_ptn})|({utf16_char_ptn}))*""#);

    let par = reg(&json_str_ptn);

    {
        let inp = r#""hello world\n"1234"#;
        let res = par.parse(inp);
        assert_eq!(Ok((r#""hello world\n""#.to_string(), "1234")), res);
    }

    {
        let inp = r#""http://www.json.org/test/123456789""#;
        let res = par.parse(inp);
        assert_eq!(
            Ok((r#""http://www.json.org/test/123456789""#.to_string(), "")),
            res
        );
    }

    {
        let inp = r#""\u2192\uD83D\uDE00\"\t\uD834\uDD1E""#;
        let res = par.parse(inp);
        assert_eq!(
            Ok((r#""\u2192\uD83D\uDE00\"\t\uD834\uDD1E""#.to_string(), "")),
            res
        );
    }

    //    println!("{:#?}", res);
}
