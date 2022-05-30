# ECS140A Project

### Due Date
05/29/2022

### Group Members
- Group member 1:
    - Name: Huilin Zhang
    - Email: hlzhang@ucdavis.edu
    - SID: 917071562
- Group member 2:
    - Name: He Li
    - Email: heili@ucdavis.edu
    - SID: 918426077

### Compile Command

You can run the program by using `cargo run ./examples/example#.x` command where `#` can be any numbers.

### Current Status
- We completed stage1, stage2, mostly stage3 but with some bugs, and partial stage4.

###### Stage1
- CStream is defined in `cstream.rs` file and it can load all contents that read in `main.rs` file and store them.

###### Stage2
- Scanner is defined in `scanner.rs` file and contains a CStream data structure that contains all content information.
- Since we have trouble in compiling stage3, **we will comment out all codes for stage3 in order to allow it compile**.
- In `scanner.rs`, we implemented functions `get_next_token()`, `get_curr_token()`, and `get_all_token`. All of them can properly function and ready to be tested.

###### Stage3
- Functions definition is under the main function. 
- Each ebnf statement defines a method, fun_program() can recurvisely call each method crossponding to ebnf statement.
- The custom error will output crosponding ebnf statement, line_num, and char_pos. 

###### Stage4
- Since stage3 couldn't be done, we assume the syntax is correct and output a partial `.html` file.
- The xhtml struct is defined in `xhtml.rs` file and takes a vector of all tokens, analyze them and output them to xhtml file in order.
- All parentheses, curly braces, brackets, and indentations are ignored / missing.


###### Reference
- Professor Jiang's solution for hw2.
- [Changing `Vec<Option<Token>>` to `Vec<Token>`](https://stackoverflow.com/questions/30588549/how-do-i-convert-a-list-of-optiont-to-a-list-of-t-when-t-cannot-be-copied)
- [Using match function instead of `if-else` statements](https://stackoverflow.com/questions/37814942/early-breaking-from-rusts-match)
- [PartialEq for comparing struct types](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
