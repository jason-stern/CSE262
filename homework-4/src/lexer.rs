
use nom::{
    IResult,
    branch::alt,
    combinator::{opt, cut},
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alpha1, alphanumeric1, digit1, space1, line_ending},

};

#[derive(Debug, Clone)]
pub enum Token {
    //Alpha(char),
    // Number(i32),
    // Boolean(bool),
    // Identifier(String),
    // String(String),

    // LeftSquareBracket { value: char},      // [
    // RightSquareBracket { value: char},     // ]
    // Slash { value: char},                  // /

    Keyword { value: String},              // let, fn, pub
    Identifier { value: String}, 
    Equals,                 // =
    NumVal { value: i32},
    BoolVal { value: bool},
    Semicolon,              // ;

    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,

    Comma,
    Operator{ value: String},
     

}

// pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
//          let tokens: Vec<Token> = many0(token(input))?;
//          Ok((input, tokens))
//     }

pub fn keyword(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, keyword) = alt((tag("let "),tag("fn "),tag("return ")))(input)?;
    //let (input, _) = many0(space1(input))?;
    Ok((input, Token::Keyword{ value: keyword.to_string() } ))
}
pub fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, identifier) = alphanumeric1(input)?;

    Ok((input, Token::Identifier{ value: identifier.to_string() }))
}
pub fn equals(input: &str) -> IResult<&str,Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, eq) = tag("=")(input)?;
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    Ok((input, Token::Equals))
}
pub fn num_val(input: &str) -> IResult<&str, Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, val) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = val.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Token::NumVal{ value: number}))                 // Return the now partially consumed input with a number as well
}
pub fn bool_val(input: &str) -> IResult<&str, Token> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Token::BoolVal{ value: bool_value}))
  }

pub fn semicolon(input: &str) -> IResult<&str,Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, eq) = tag(";")(input)?;
    Ok((input, Token::Semicolon))
}
pub fn left_paren(input: &str) -> IResult<&str,Token> {
    let (input, paren) = tag("(")(input)?;
    Ok((input, Token::LeftParen))
}
pub fn right_paren(input: &str) -> IResult<&str,Token> {
    let (input, paren) = tag(")")(input)?;
    Ok((input, Token::LeftParen))
}
pub fn left_curly(input: &str) -> IResult<&str,Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, paren) = tag("{")(input)?;
    Ok((input, Token::LeftCurly))
}
pub fn right_curly(input: &str) -> IResult<&str,Token> {
    let (input, _) = many0(alt((space1,line_ending)))(input)?;
    let (input, paren) = tag("}")(input)?;
    Ok((input, Token::RightCurly))
}
pub fn comma(input: &str) -> IResult<&str, Token> {
    let (input, paren) = tag(",")(input)?;
    Ok((input, Token::Comma))
}
pub fn operator(input: &str) -> IResult<&str, Token> {
    let (input, op) = alt((tag("+"),tag("-"),tag("*"),tag("/")))(input)?;
    Ok((input, Token::Operator{ value: op.to_string()}))
    
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, result) = many1(alt((keyword,num_val,bool_val,identifier,equals,semicolon,left_paren,right_paren,left_curly,
                                    right_curly,comma,operator)))(input)?;
    Ok((input, result))
  }
/*
    Below: Parser
*/
#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  //LogicExpression { name: String, children: Vec<Node> },
  IfExpression { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  MathAdd {children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
  Null,
}

pub fn function_definition(input: Vec<Token>) -> IResult<Vec<Token>, Node> {
  let keyword = input[0];
}
pub fn statement(input: Vec<Token>) -> IResult<Vec<Token>, Node> {
    
}
pub fn expression(input: Vec<Token>) -> IResult<Vec<Token>, Node> {
    
}
pub fn program(input: Vec<Token>) -> IResult<Vec<Token>, Node> {
    
}