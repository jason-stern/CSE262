extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, start_interpreter};
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected);
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));

// Final exam - tests for comparative operators
test!(comparative_op_true, r#"
fn main() {
  let x = 5 == 5;
  return x;
}
"#, Ok(Value::Bool(true)));
test!(comparative_op_num_then_expression, r#"
fn main() {
  let x = 3;
  if 3 == x {
    return 25;
  }
}
"#, Ok(Value::Number(25)));
test!(compare_greaterthan, r#"
fn main() {
  return 5 > 1;
}
"#, Ok(Value::Bool(true)));
test!(compare_lessthan, r#"
fn main() {
  return 1 < 10;
}
"#, Ok(Value::Bool(true)));
test!(compare_greaterthanorequals, r#"
fn main() {
  return 10 >= 10;
}
"#, Ok(Value::Bool(true)));
test!(compare_lessthanorequals, r#"
fn main() {
  return 10 <= 10;
}
"#, Ok(Value::Bool(true)));
test!(comparative_op_math_expression, r#"
fn main() {
  let x = 10;
  let y = 5;
  let z = 3;
  let result = x + 5 == 18 - z;
  return result;
}
"#, Ok(Value::Bool(true)));
test!(comparative_op_math_expression_false, r#"
fn main() {
  let x = 1 + 2 == 4;
  return x;
}
"#, Ok(Value::Bool(false)));
test!(comparative_op_false, r#"
fn main() {
  let x = 5 == 6;
  return x;
}
"#, Ok(Value::Bool(false)));
test!(invalid_comparison, r#"
fn main() {
  let x = 5 == true;
}
"#, Err("Invalid expression - can only compare numbers to numbers"));
test!(invalid_comparison_2, r#"
fn main() {
  let x = 5 != true;
}
"#, Err("Invalid expression - can only compare numbers to numbers"));
test!(invalid_comparison_3, r#"
fn main() {
  let x = 5 != false;
}
"#, Err("Invalid expression - can only compare numbers to numbers"));


// Final exam - tests for If Statements
test!(if_statement_true, r#"
fn main() {
  let x = 3;
  if x == 3 {
    return 25;
  }
}
"#, Ok(Value::Number(25)));
test!(if_statement_false, r#"
fn main() {
  let x = 3;
  if x == 4 {
    return 25;
  }
}
"#, Ok(Value::Bool(true)));
test!(if_else, r#"
fn main() {
  let x = 3;
  if x == 4 {
    return 25;
  } else {
    return 10;
  } 
}
"#, Ok(Value::Number(10)));
test!(if_else_if, r#"
fn main() {
  let x = 10;
  if x == 5 {
    return 1;
  } else if x == 6 {
    return 2;
  } else if x == 10 {
    return 3;
  }
}
"#, Ok(Value::Number(3)));
test!(if_elseif_else, r#"
fn main() {
  let x = 0;
  if x == 5 {
    return 1;
  } else if x == 6 {
    return 2;
  } else if x == 10 {
    return 3;
  } else {
    return 4;
  }
}
"#, Ok(Value::Number(4)));
test!(if_with_functioncall, r#"
fn timestwo(x) {
  return x * 2;
}
fn main() {
  let x = 5;
  if x == 5 {
    return timestwo(10);
  }
}
"#, Ok(Value::Number(20)));
test!(if_with_comparisons, r#"
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
"#, Ok(Value::Number(102)));
test!(x_equals_true, r#"
fn main() {
  let x = true;
  if x == true {
    return 2;
  }
}
"#, Ok(Value::Number(2)));