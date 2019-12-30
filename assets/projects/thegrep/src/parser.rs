use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thegrep - Tar Heel egrep - Parser
 *
 * @file parser.rs
 * @brief Parser for the grammar of thegrep.
 *
 * Author: Austin Hale
 * ONYEN: haleau
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/* == Begin Syntax Tree Elements == */
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    OneOrMore(Box<AST>),
    Char(char),
    AnyChar,
}

/* Helper factory functions for building ASTs */
pub fn to_alternation(lhs: AST, rhs: AST) -> AST {
    AST::Alternation(Box::new(lhs), Box::new(rhs))
}

pub fn to_catenation(lhs: AST, rhs: AST) -> AST {
    AST::Catenation(Box::new(lhs), Box::new(rhs))
}

pub fn to_closure(ast_box: AST) -> AST {
    AST::Closure(Box::new(ast_box))
}

pub fn to_char(value: char) -> AST {
    AST::Char(value)
}

pub fn to_one_or_more(ast_box: AST) -> AST {
    AST::OneOrMore(Box::new(ast_box))
}

/* == End Syntax Tree Elements == */

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    /**
     * @brief Takes the tokens from the tokenizer when initializing
     * the parser and returns the ASTession Result.
     */
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };
        // Peeks the parser's next token to ensure no remaining tokens
        // in the parser after parsing the AST enum.
        let parse_expr = parser.reg_expr();
        if let Some(token) = parser.tokens.peek() {
            Err(format!("Expected end of input, found {:?}", token))
        } else {
            parse_expr
        }
    }
}

// Unit Tests for main aspects of thegrep's grammar //
#[cfg(test)]
mod public_api {
    use super::*;

    // The following valid tests for thegrep should pass for the grammar.
    #[test]
    fn parse_catenate_closure() {
        let _res = Parser::parse(Tokenizer::new("a.*")).unwrap();
        assert_eq!(to_catenation(to_char('a'), to_closure(AST::AnyChar)), _res);
    }

    #[test]
    fn parse_catenation() {
        let _res = Parser::parse(Tokenizer::new("aaa")).unwrap();
        assert_eq!(
            to_catenation(to_char('a'), to_catenation(to_char('a'), to_char('a'))),
            _res
        );
    }

    #[test]
    fn parse_grouped_alternation() {
        let _res = Parser::parse(Tokenizer::new("1|2|3")).unwrap();
        assert_eq!(
            to_alternation(to_char('1'), to_alternation(to_char('2'), to_char('3'))),
            _res
        );
    }

    #[test]
    fn parse_test_parentheses() {
        let _res = Parser::parse(Tokenizer::new("(1)|(2)|(3)")).unwrap();
        assert_eq!(
            to_alternation(to_char('1'), to_alternation(to_char('2'), to_char('3'))),
            _res
        );
    }

    // The following tests should panic given the specifications
    // in the thegrep grammar.
    #[test]
    #[should_panic]
    fn parse_fail_missing_parentheses() {
        let _res = Parser::parse(Tokenizer::new("(ab")).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_fail_no_left_parentheses() {
        let _res = Parser::parse(Tokenizer::new("a*)")).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_fail_missing_union() {
        let _res = Parser::parse(Tokenizer::new("|b")).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_fail_missing_alternation() {
        let _res = Parser::parse(Tokenizer::new("a|")).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_fail_extra_kleene() {
        let _res = Parser::parse(Tokenizer::new("a**")).unwrap();
    }
}

/**
 * Internal-only parser methods to process the grammar via recursive descent.
 */
impl<'tokens> Parser<'tokens> {
    // RegExpr ::= <Catenation> (UnionBar <RegExpr>)?
    /**
     * @brief Continues to the catenation part of the grammar, then discovers
     * if there are zero or one alternations for the catenation.
     * @returns Returns the AST Alternation Token on success and otherwise
     * returns only the catenation or a specific error.
     */
    fn reg_expr(&mut self) -> Result<AST, String> {
        let catenation = self.catenation()?;

        // If there is no UnionBar, uses the catenation as the base case.
        match self.peek_union_bar() {
            Ok(_next) => match self.consume_token(Token::UnionBar) {
                Ok(_next) => match self.reg_expr() {
                    Ok(reg_expr) => Ok(to_alternation(catenation, reg_expr)),
                    Err(_e) => Err(String::from("Missing RegExpr after UnionBar")),
                },
                Err(_e) => Err(String::from("Should always consume the union bar here")),
            },
            Err(_e) => Ok(catenation),
        }
    }

    // Catenation ::= <Closure> <OneOrMore> <Catenation>?
    /**
     * @brief Continues to Closure and OneOrMore in the grammar and discovers
     * if there are zero or more Catenations.
     * @returns Returns the AST Catenation Token with the Closure
     * and Atom on success. Otherwise, this function will return
     * only the OneOrMore.
     */
    fn catenation(&mut self) -> Result<AST, String> {
        let closure = self.closure()?;
        let one_or_more = self.one_or_more(closure)?;
        match self.is_not_atom_beginning() {
            Ok(_e) => Ok(one_or_more),
            Err(_e) => {
                if let Ok(atom) = self.catenation() {
                    Ok(to_catenation(one_or_more, atom))
                } else {
                    Ok(one_or_more)
                }
            }
        }
    }

    // Closure ::= <Atom> KleeneStar?
    /**
     * @brief Continues to Atom in the grammar and discovers
     * if there are zero or one KleeneStar Tokens following it.
     * @returns Returns the AST Closure Token with the Atom on success
     * and otherwise returns just the atom or the specific error.
     */
    fn closure(&mut self) -> Result<AST, String> {
        let atom = self.atom()?;
        match self.peek_kleene_star() {
            Ok(_c) => match self.take_next_token() {
                Ok(_e) => Ok(to_closure(atom)),
                Err(e) => Err(e),
            },
            Err(_e) => Ok(atom),
        }
    }

    // OneOrMore ::= KleenePlus?
    /**
     * @brief Checks to see if there is a following KleenePlus Token.
     * @returns Returns AST OneOrMore Token with the Closure/Atom
     * on success and otherwise returns the Closure/Atom.
     */
    fn one_or_more(&mut self, closure: AST) -> Result<AST, String> {
        match self.peek_kleene_plus() {
            Ok(_c) => match self.take_next_token() {
                Ok(_e) => Ok(to_one_or_more(closure)),
                Err(e) => Err(e),
            },
            Err(_e) => Ok(closure),
        }
    }

    // Atom ::= LParen <RegExpr> RParen | AnyChar | Char
    /**
     * @brief Takes the next token to see if it begins as an atom.
     * @returns Returns the AST Token on success and otherwise
     * specifies the error.
     */
    fn atom(&mut self) -> Result<AST, String> {
        let t: Token = self.take_next_token()?;
        match t {
            Token::LParen => self.opt_parentheses(),
            Token::AnyChar => Ok(AST::AnyChar),
            Token::Char(c) => Ok(to_char(c)),
            _ => Err(format!("Unexpected token: {:?}", t)),
        }
    }

    /**
     * @brief Recursive helper function for handling atoms
     * that begin with a left parentheses.
     * @returns Returns the regular exp_ression as an atom if
     * successful and otherwise specifies the error.
     */
    fn opt_parentheses(&mut self) -> Result<AST, String> {
        let expr = self.reg_expr();
        match &expr {
            Ok(_next) => match self.consume_token(Token::RParen) {
                Ok(_next) => expr,
                Err(e) => Err(e),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

// Helper method tests located inside the parser //
#[cfg(test)]
mod private_api {
    use super::*;

    // The following three tests ensure the functionality of the reg_expr
    // helper function.
    #[test]
    fn parse_reg_expr_helper() {
        let _res = Parser::from("a.*").reg_expr().unwrap();
        assert_eq!(to_catenation(to_char('a'), to_closure(AST::AnyChar)), _res);
    }

    #[test]
    fn parse_reg_expr_catenation() {
        let _res = Parser::from("aba").reg_expr().unwrap();
        assert_eq!(
            to_catenation(to_char('a'), to_catenation(to_char('b'), to_char('a'))),
            _res
        );
    }

    #[test]
    #[should_panic]
    fn parse_missing_rside_reg_expr() {
        let _res = Parser::parse(Tokenizer::new("1|")).unwrap();
    }

    // The following three tests ensure the catenation aspect of
    // thegrep's grammar.
    #[test]
    fn parse_catenation_helper() {
        let _res = Parser::from("a.a").catenation().unwrap();
        assert_eq!(
            to_catenation(to_char('a'), to_catenation(AST::AnyChar, to_char('a'))),
            _res
        );
    }

    #[test]
    fn parse_catenation_recursive_helper() {
        let _res = Parser::from("a.a.a").catenation().unwrap();
        assert_eq!(
            to_catenation(
                to_char('a'),
                to_catenation(
                    AST::AnyChar,
                    to_catenation(to_char('a'), to_catenation(AST::AnyChar, to_char('a')))
                )
            ),
            _res
        );
    }

    #[test]
    fn parse_fail_catenation_beginning_atom() {
        let _res = Parser::from("a)").catenation().unwrap();
        assert_eq!(to_char('a'), _res);
    }

    // The following three tests ensure the closure aspect of the grammar.
    #[test]
    fn parse_closure_helper() {
        let _res = Parser::from("a*").catenation().unwrap();
        assert_eq!(to_closure(to_char('a')), _res);
    }

    #[test]
    fn parse_closure_parentheses() {
        let _res = Parser::from("(a|b)*").catenation().unwrap();
        assert_eq!(to_closure(to_alternation(to_char('a'), to_char('b'))), _res);
    }

    #[test]
    fn parse_closure_no_kleene_star() {
        let _res = Parser::from("a").catenation().unwrap();
        assert_eq!(to_char('a'), _res);
    }

    // The following tests ensure the creation of atoms in thegrep's grammar.
    #[test]
    fn parse_atom_helper() {
        let _res = Parser::from(".").atom().unwrap();
        assert_eq!(AST::AnyChar, _res);
    }

    #[test]
    fn parse_atom_helper_any_char() {
        let _res = Parser::from("c").atom().unwrap();
        assert_eq!(to_char('c'), _res);
    }

    #[test]
    #[should_panic]
    fn parse_atom_helper_invalid_char() {
        let _res = Parser::from("|").atom().unwrap();
    }
}

/* Parser's Helper Methods to improve ergonomics of parsing */
impl<'tokens> Parser<'tokens> {
    /**
     * Static helper method used in unit tests to establish a
     * parser given a string.
     */
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    /**
     * @brief Helper function used to see if the subsequent token
     * could be an atom.
     * @returns Returns an Ok _result if the next token could NOT begin
     * another Atom and otherwise returns an error.
     */
    fn is_not_atom_beginning(&mut self) -> Result<(), String> {
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::RParen | Token::UnionBar | Token::KleeneStar => Ok(()),
                _ => Err(String::from(
                    "Next token could be the beginning of an Atom in helper",
                )),
            }
        } else {
            Err(String::from(
                "There are no remaining tokens to peek in atom helper",
            ))
        }
    }

    /**
     * @brief Taking the next token directly or raise an error that
     * you expected another token here but found the end of input.
     */
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    /**
     * @brief Peeks the next token for a UnionBar. Returns the
     * Token type if found and otherwise specifies the error.
     */
    fn peek_union_bar(&mut self) -> Result<Token, String> {
        if let Some(Token::UnionBar) = self.tokens.peek() {
            Ok(Token::UnionBar)
        } else {
            Err(String::from("Expected union operator in parser helper"))
        }
    }

    /**
     * @brief Peeks the next token for a KleeneStar. Returns the
     * Token type if found and otherwise specifies the error.
     */
    fn peek_kleene_star(&mut self) -> Result<Token, String> {
        if let Some(Token::KleeneStar) = self.tokens.peek() {
            Ok(Token::KleeneStar)
        } else {
            Err(String::from("Expected kleene star in parser helper"))
        }
    }

    /**
     * @brief Peeks the next token for a KleeneStar. Returns the
     * Token type if found and otherwise specifies the error.
     */
    fn peek_kleene_plus(&mut self) -> Result<Token, String> {
        if let Some(Token::KleenePlus) = self.tokens.peek() {
            Ok(Token::KleenePlus)
        } else {
            Err(String::from("Expected kleene plus in parser helper"))
        }
    }

    /**
     * @brief Helper method when there's a specific token you expect
     * next in the grammar. Raises an Err if there is no next token
     * or if it is not _exactly_ the Token you expected next.
     * @returns If it is the token you expected, it will return Ok(Token).
     */
    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }
}
