# thegrep
Command line program that utilizes lexemes and a grammar to replicate the egrep program on most Linux/Unix devices.

## Getting Started
Lexemes of **thegrep**:
```
<LParen>      ::= '('
<RParen>      ::= ')'
<UnionBar>    ::= '|'
<KleeneStar>  ::= '*'
<AnyChar>     ::= '.'
<Char>        ::= any character except '(', ')', '|', '.', '*'
```
Grammar of **thegrep**:
```
<RegExpr>     ::= <Catenation> <UnionBar <RegExpr>>?
<Catenation>  ::= <Closure> <Catenation>?
<Closure>     ::= <Atom> KleeneStar?
<Atom>        ::= LParen <RegExpr> RParen | AnyChar | Char
```

## Usage
```
thegrep [FLAGS] <pattern>

FLAGS:
  -h, --help      Prints help information
  -p, --parse     Show Parsed AST
  -t, --tokens    Show Tokens
  -d, --dot       Show Dot Diagram
  -V, --version   Prints version information

ARGS:
  <pattern>     Regular Expression Pattern
```
With Rust's package manager Cargo, you could also run ```cargo run -- [FLAGS] <pattern>```.

## Overall Design Implementation

### Organization
The overall organization of this project consists of three files and each with their own distinct task:
 - [main.rs](src/main.rs) handles the command line options for processing input. 
 - [tokenizer.rs](src/tokenizer.rs) acts as a tokenizer for identifying and consuming each lexeme in thegrep.
 - [parser.rs](src/parser.rs) handles thegrep's grammar in how non-terminals coincide with one another.

### Accomplishments
The aspects of this design that I am particularly proud of include the organization within the Tokenizer and Parser. By taking input in the main file, I was able to separate each flag's processes in a succinct manner. With these files, I split the core evaluation and helper methods. In addition, I found the design decision to contain factory functions for the Parser extremely efficient when conducting test code. 

### Additional Notes
As far as testing is concerned, there are public and private tests located in the Tokenizer and Parser. These tests ensure the validity of public and private functions. They also panic for invalid instances in thegrep's lexemes and grammar.
