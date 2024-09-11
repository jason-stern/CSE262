extern crate nom;
extern crate asalang;

use asalang::{program, start_interpreter};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"
  fn main() {
    let x = 2;
    let y = 5;
    let z = 10;
    if x + 2 == y - 6 {
      return 100;
    } else if x == 3 {
      return 101;
    } else if z - 4 == 6 {
      return 102;
    } else {
      return 103;
    }
  }
  "#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      let result = start_interpreter(&tree);
      println!("{:?}", result);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }

    
  Ok(())
}
