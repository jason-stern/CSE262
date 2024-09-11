# CSE262 - Programming Languages - Fall 2023

# Homework 2 - Lexer

**Due Date: 10/4/2023 EOD**

*Important: For this assignment you must make at least 4 commits to your repository.*

## Introduction

In this assignment, we will implement the lexical analysis stage of our Asa compiler, called a "lexer". The idea of the lexer is to categorize every byte of the input source code.

In `src\lib.rs`, you'll find a definition for the `Token` enum, and the `lex()` function. You will assign the following categories to the the bytes of an ASCII encoded input string.

- Alpha - any alphabetical character, upper or lower case
- Digit - 0 to 9
- Whitespace - space, tab, newline, carriage return
- Grouping Symbols - left and right parens and curly braces
- Operators - either `+` or `-`
- Equal - the equal character `=`
- Quote - a double quotation character `"`
- Semicolon - a semicolon character `;`
- Keyword - one of:
  - `true`
  - `false`
  - `fn`
  - `return`
  - `let`
- EOF - the end of file token will always be placed at the end of the token vector
- Other - Any byte that doesn't fall into the above categories will be considered as an "other" token.

## lex()

Implement the `lex()` function in `src\lib.rs`. You will find a number of tests in `tests\lex.rs` that you must pass in order to implement this function.

The function should iterate over every byte of the input string, and categorize each one according to the token descriptions above.

Most tokens map to a single byte, but keywords map to multiple bytes; e.g. `let` shouldn't be tokenized as `[Token::Alpha, Token::Alpha, Token::Alpha]` but as `[Token::Keyword]`.

You may find an [ASCII table](http://www.asciitable.com) and the match statement to help you with this task.

## Storing Bytes on Enum Variants


```rust
pub enum Token {
  True,
  False,
  Fn,
  Return,
  Let,
  Alpha,
  Digit,
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  Equal,
  Plus,
  Dash,
  Quote,
  WhiteSpace,
  Semicolon,
  Comma,
  Other,
  EOF,
}
```
You can now store the byte(s) associated with the enum variant on the variant itself. Update your `lex()` function to accomodate this change. Furthermore, you'll need to update the tests as well.

## Video Explanation
Link: https://drive.google.com/file/d/1M7tlgwMKIKN0AFlJ-5gkd4JxQwxoFu2b/view?usp=sharing

Create a screen recording of your lexer, and demonstrate that it works. As part of your video, you should hit the following points:

1. Describe what a lexer is and what its place is in the compilation pipeline. 
2. Describe how you implemented the `lex()` function.
3. Some character encodings use more than one byte per visible character (*e.g.* [Unicode](https://en.wikipedia.org/wiki/Unicode)). How might you modify your lexer to handle Unicode characters?

To demonstrate it working, it's sufficient to show the tokenized output that is produced by the `lex()` function. Be sure to explain why the output you show is the correct output.

If you did not successfully write the lexer, you may show output for as far as you were able to get. Also, explain how you attempted to solve this assignment and describe where you got stuck. Show off any code you did write.

[Here is a link](https://support.zoom.us/hc/en-us/articles/201362473-Local-recording) to some instructions for using Zoom to create a recording. 

If you need any accomodations with this part of the assignment, please see me ASAP.

## Assignment Instructions

1. Fork the relevant repository into your CSE262 group namespace.

2. Clone your newly forked repository to your computer. Your repository should be hosted at 
```
http://gitlab.cse.lehigh.edu/<your user name>-cse262/<assignment name>
```
You can use the following git command with the appropriate values substituted to customize it for you:
```
git clone http://gitlab.cse.lehigh.edu/<your user name>-cse262/<assignment name>
```
[Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 

3. Write the necessary code to answer all questions. Commit your progress periodically.

4. Push your work to the remote Gitlab servers using the `git push` command.

5. Verify that your commits were successfully pushed to Gitlab by visiting the repository on the website and noting the latest commit is the one you just pushed.
