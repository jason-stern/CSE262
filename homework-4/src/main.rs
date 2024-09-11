extern crate nom;
extern crate asalang_parser;

use asalang_parser::{tokenize, program, Token, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  let token_vec = tokenize(r#"let x = 5;"#);
  match token_vec {
    Ok((unparsed,tokens)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Tokens:\n {:#?}", tokens);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
      return 10;
    }
  }

  let result = program(token_vec);
  match result {
    Ok((unparsed,tokens)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Tokens:\n {:#?}", tokens);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }


    
  Ok(())
}
