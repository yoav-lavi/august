use colored::Colorize;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, value},
    error::{context, ContextError, ErrorKind, ParseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use serde::Serialize;
use serde_json as _;
use std::collections::HashMap;
use std::str;
use toml as _;

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn parse_str<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric, '\\', one_of("\"n\\"))(input)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false)).parse(input)
}

fn null<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), tag("null")).parse(input)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    context("string", preceded(char('\"'), cut(terminated(parse_str, char('\"')))))(input)
}

fn unquoted_string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(char::is_alphanumeric)(input)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<JsonValue>, E> {
    context(
        "array",
        preceded(char('>'), cut(separated_list0(char('+'), json_value))),
    )(input)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
    separated_pair(alt((string, unquoted_string)), cut(char(':')), json_value).parse(input)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, HashMap<String, JsonValue>, E> {
    context(
        "map",
        preceded(
            char('.'),
            cut(terminated(
                map(separated_list0(char('+'), key_value), |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(key, value)| (String::from(key), value))
                        .collect()
                }),
                alt((tag("^"), tag("\n"), tag(""))),
            )),
        ),
    )(input)
}

fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(input: &'a str) -> IResult<&'a str, JsonValue, E> {
    alt((
        map(hash, JsonValue::Object),
        map(array, JsonValue::Array),
        map(alt((string, unquoted_string)), |string| {
            JsonValue::Str(string.to_owned())
        }),
        map(double, JsonValue::Num),
        map(boolean, JsonValue::Boolean),
        map(null, |_| JsonValue::Null),
    ))
    .parse(input)
}

fn root<'a, E: ParseError<&'a str> + ContextError<&'a str>>(input: &'a str) -> IResult<&'a str, JsonValue, E> {
    alt((
        map(hash, JsonValue::Object),
        map(array, JsonValue::Array),
        map(null, |_| JsonValue::Null),
    ))
    .parse(input)
}

fn main() {
    let data = r#"

    .semi:true+trailingComma:all+singleQuote:true+printwidth:120+tabwidth:2+ignored:.hello:world

    "#
    .trim();

    println!("\n{}\n\n{data}\n\n{}\n", "Input".bright_cyan(), "Output".bright_cyan());

    let result = root::<(&str, ErrorKind)>(data);

    let output = result.unwrap().1;

    println!(
        "{}\n\n{}",
        "---JSON---".bright_green(),
        serde_json::to_string_pretty(&output).expect("must parse")
    );

    println!();

    println!(
        "{}\n\n{}",
        "---TOML---".bright_green(),
        toml::to_string_pretty(&output).expect("must parse")
    );

    println!();

    println!(
        "{}\n\n{}",
        "---YAML---".bright_green(),
        serde_yaml::to_string(&output).expect("must parse")
    );
}
