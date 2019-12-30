use std::iter::Peekable;
use std::str::Chars;

/**
 * thegrep's Tokenizer
 *
 * @file tokenizer.rs
 * @brief A tokenizer for the consumption of lexemes in the thegrep language.
 *
 * Author: Austin Hale
 * ONYEN: haleau
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * Derives from Debug and from PartialEq for using the debug placeholder
 * and assert_eq! in tests, respectively.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    UnionBar,
    KleeneStar,
    AnyChar,
    Char(char),
    KleenePlus,
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a &str's Chars.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

/**
 * The public facing impl of Tokenizer.
 */
impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer in this impl.
 * It will produce items of type Token and has a 'next' method that
 * returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * @brief While ignoring whitespace, this function peeks through
     * each character and consumes in its specific helper method.
     * @returns Returns the Option of the next token value.
     */
    fn next(&mut self) -> Option<Token> {
        self.lex_whitespace();
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '(' | ')' => self.lex_paren(),
                '|' => self.lex_union_bar(),
                '*' => self.lex_kleene_star(),
                '.' => self.lex_any_char(),
                '+' => self.lex_kleene_plus(),
                _ => self.lex_char(),
            })
        } else {
            None
        }
    }
}

/**
 * @brief Unit Tests for the `next` method.
 *
 * @details Checks a variety of token inputs for correctly
 * matching the iterator's next value.
 */
#[cfg(test)]
mod iterator {
    use super::*;

    // This test assures that there is no ambiguity with a missing input.
    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    // This test asserts the foundation of handling an unknown char token.
    #[test]
    fn unknown_char() {
        let mut tokens = Tokenizer::new("k");
        assert_eq!(tokens.next(), Some(Token::Char('k')));
        assert_eq!(tokens.next(), None);
    }

    // This test asserts the simple handling of parentheses tokens.
    #[test]
    fn print() {
        let mut tokens = Tokenizer::new("()(");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), None);
    }

    // A starry test that ensures the readability of the other tokens.
    #[test]
    fn star_eyes_tokens() {
        let mut tokens = Tokenizer::new("*_* .|.  *_*");
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::Char('_')));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::Char('_')));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    // This test offers various spacing between different token types.
    #[test]
    fn check_spaces() {
        let mut tokens = Tokenizer::new("(      *     f      ");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::Char('f')));
        assert_eq!(tokens.next(), None);
    }

    // This test checks the tokenizer's ability to ignore whitespace
    // and tab placement.
    #[test]
    fn check_tabs() {
        let mut tokens = Tokenizer::new("\t* \t*\t\t((");
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), None);
    }

    // This test uses all possible tokens (|, (, ), *, .) to
    // ensure that the iterator does not return a char type.
    #[test]
    fn check_operators() {
        let mut tokens = Tokenizer::new("()|*|.");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }
}

/**
 * Helper methods of Tokenizer are in the impl block below.
 */
impl<'str> Tokenizer<'str> {
    /**
     * @brief Consumes unnecessary whitespace given to the tokenizer.
     */
    fn lex_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }

    /**
     * @brief Differentiates between left and right parentheses
     * and consumes the token.
     * @returns Left or Right Parentheses token.
     */
    fn lex_paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("unknown register"),
        }
    }

    /**
     * @brief Consumes the subsequent union bar lexeme.
     * @returns Returns the UnionBar Token.
     */
    fn lex_union_bar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("Unexpected union bar helper"),
        }
    }

    /**
     * @brief Consumes the next KleeneStar Token.
     * @returns Returns the KleeneStar Token.
     */
    fn lex_kleene_star(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("Unexpected kleene star helper"),
        }
    }

    /**
     * @brief Ensures the consumption of the next AnyChar Token.
     * @returns Returns the AnyChar Token.
     */
    fn lex_any_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("Unexpected any char helper"),
        }
    }

    /**
     * @brief Panics if the character is already a lexeme and otherwise
     * consumes the next Char Token.
     * @returns Returns the specified Char Token.
     */
    fn lex_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' | ')' | '|' | '*' | '.' => panic!("Exists as a lexeme"),
            _ => Token::Char(c),
        }
    }

    /**
     * @brief Consumes the next KleenePlus Token.
     * @returns Returns the KleenePlus Token.
     */
    fn lex_kleene_plus(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '+' => Token::KleenePlus,
            _ => panic!("Unexpected kleene plus helper"),
        }
    }
}

/**
 * Unit Tests for helper methods.
 */
#[cfg(test)]
mod helper_method {
    // Import identifiers from containing module
    use super::*;

    /// Helper Method Tests ///

    // Ensures the lexeme whitespace helper function takes all whitespace
    // and tab/newline characters.
    #[test]
    fn take_lex_whitespace() {
        let mut tokens = Tokenizer::new("      \t \t   \n");
        tokens.lex_whitespace();
        assert_eq!(tokens.chars.next(), None);
    }

    // This test ensures the lex_paren distinguishes and consumes
    // each specified left and right parentheses.
    #[test]
    fn take_lex_paren() {
        let mut tokens = Tokenizer::new("(()))");
        assert_eq!(tokens.lex_paren(), Token::LParen);
        assert_eq!(tokens.lex_paren(), Token::LParen);
        assert_eq!(tokens.lex_paren(), Token::RParen);
        assert_eq!(tokens.lex_paren(), Token::RParen);
        assert_eq!(tokens.lex_paren(), Token::RParen);
        assert_eq!(tokens.chars.next(), None);
    }

    // This test ensures the consumption of a '|' token.
    #[test]
    fn take_lex_union() {
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.lex_union_bar(), Token::UnionBar);
        // Ensure we consumed the unknown character
        assert_eq!(tokens.chars.next(), None);
    }

    // This function tests the consumption of '*' for the Kleene Star token.
    #[test]
    fn take_lex_kleene_star() {
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.lex_kleene_star(), Token::KleeneStar);
        assert_eq!(tokens.chars.next(), None);
    }
    
    // This function tests the consumption of '+' for the Kleene Plus token.
    #[test]
    fn take_lex_kleene_plus() {
        let mut tokens = Tokenizer::new("+");
        assert_eq!(tokens.lex_kleene_plus(), Token::KleenePlus);
        assert_eq!(tokens.chars.next(), None);
    }

    // This function tests the consumption of the AnyChar token.
    #[test]
    fn take_any_char() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.lex_any_char(), Token::AnyChar);
        assert_eq!(tokens.chars.next(), None);
    }

    // Ensures that taking a non-print character panics.
    #[test]
    #[should_panic]
    fn test_paren_panics() {
        let mut tokens = Tokenizer::new("()a");
        tokens.lex_paren();
        tokens.lex_paren();
        tokens.lex_paren();
    }

    // Ensures that taking a character other than '|' panics.
    #[test]
    #[should_panic]
    fn test_union_bar_panics() {
        let mut tokens = Tokenizer::new("| a");
        assert_eq!(tokens.lex_union_bar(), Token::UnionBar);
        tokens.lex_union_bar();
    }

    // Ensures that taking a character other than '*' panics.
    #[test]
    #[should_panic]
    fn test_kleene_star_panics() {
        let mut tokens = Tokenizer::new("* b");
        assert_eq!(tokens.lex_kleene_star(), Token::KleeneStar);
        tokens.lex_kleene_star();
    }

    // Ensures that taking a character other than '.' panics.
    #[test]
    #[should_panic]
    fn test_any_char_panics() {
        let mut tokens = Tokenizer::new(". c");
        assert_eq!(tokens.lex_any_char(), Token::AnyChar);
        tokens.lex_any_char();
    }

    // Ensures that taking a lexeme in lex_char panics the program.
    #[test]
    #[should_panic]
    fn test_operators_panics() {
        let mut tokens = Tokenizer::new("*/-+a");
        tokens.lex_char();
    }
}
