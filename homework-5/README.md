# CSE262 - Programming Languages

# Homework 5 - Writing an Interpreter

**⏰ Due Date: 12/27/2023 EOD**

## Instructions 

**Read thoroughly before starting your project:**

1. Fork this repository into your CSE262 project namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Clone your newly forked repository onto your development machine. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
3. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work. 

**💥IMPORTANT: You must commit frequently as you work on your project. As a general rule try to make at least one commit per function you implement.**

## Assignment

In this project you will write a grammar and a parser that recognizes the grammar. I've provided a number of statements in the language in `src/parser.rs`, which runs tests against your code. There are 28 tests you will need to pass to get full credit. You can think of each test as a question. If you write code that passes more than one test, just make a dummy commit to satisfy the commit requirement.

An example program in the Asa language looks like this:

```
fn foo(a,b,c) {
  let x = a + 1; 
  // This is a comment
  let y = bar(c - b);
  return x + y; // Add the results
}

fn bar(a) {
  return a + 3;
}

fn main() {
  return foo(1,2,3);  
}
```

1. `src/interpreter.rs` contains a partial implementation of the interpreter, which includes a `Value` enum, a `Runtime` struct, and an implementation of the `run()` function.

2. Th `Value` enum represents the different types of values that the interpreter can handle, including strings, numbers, and booleans.
The `Runtime` struct represents the state of the interpreter at any given time, including the set of defined functions and the current stack of variable bindings.

2. The `run()` function is the main entry point for the interpreter, and is responsible for interpreting an AST (abstract syntax tree) produced by the parser.

3. You will need to complete the implementation of the interpreter by filling in the various match arms in the run function to handle the different types of AST nodes.

4. The comments in the code provide some guidance on what each match arm should do, but you may need to refer to the grammar and parser to understand the structure of the AST nodes.

5. As you work on the implementation, continually run the tests to ensure that the interpreter is working correctly. You can use the tests provided in `test/tests.rs` as a starting point, but you may need to write additional tests to cover specific cases.

## Deliverables

Finish the interpreter that's started in `/src/interpreter.rs`. 

Write 5 more tests in `test/tests.rs`. Make sure they increase test coverage; you want to add tests to verify things tha aren't tested already.

Create a screen recording of your lexer, and demonstrate that it works. As part of your video, you should hit the following points:

- Describe what an interpreter its place is in the program execution pipeline.
- Describe how you implemented the exec() function.
- How might you extend this interpreter in the future?

To demonstrate it working, it's sufficient to show the output that is produced by the exec() function. Be sure to explain why the output your interpreter prints is the correct output.

If you did not successfully write the interpreter, you may show output for as far as you were able to get. Also, explain how you attempted to solve this assignment and describe where you got stuck. Show off any code you did write.

If you need any accomodations with this part of the assignment, please see me ASAP.
