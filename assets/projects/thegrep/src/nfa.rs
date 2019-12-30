pub mod helpers;

/**
 * thegrep - Tar Heel egrep
 *
 * @file nfa.rs
 * @brief Recursive implementation for constructing and simulating
 * an NFA.
 *
 *
 * Author(s): Austin Hale
 * ONYEN(s): haleau
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */
extern crate rand;
extern crate structopt;

use self::State::*;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::ops::Add;

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * @brief Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     * @returns Returns true/false based on success/failure of reaching
     * the final End State.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let output_vec: Vec<char> = input.chars().collect();
        self.recursive_state(&output_vec, 0, 0)
    }

    /**
     * @brief Traverses through the current NFA and utilizes a random
     * boolean to next states and random char for the AnyChar token.
     * @returns Returns a randomly generated string that is acceptable
     * by the NFA.
     */
    pub fn gen(&self) -> String {
        // Random thread for generating an Alphanumeric character.
        let mut rng = rand::thread_rng();
        let mut output = String::from("");
        let mut current_state = 0;

        // Continue through the states until it reaches the final End State.
        loop {
            match &self.states[current_state] {
                Start(next) => {
                    if let Some(n) = next {
                        current_state = *n;
                    }
                }
                Match(c, next) => match c {
                    Char::Any => {
                        output.push(rng.sample(Alphanumeric));
                        current_state = next.unwrap();
                    }
                    Char::Literal(character) => {
                        output.push(*character);
                        current_state = next.unwrap();
                    }
                },
                Split(lhs, rhs) => {
                    // Utilizes the external rand library for a random true/false
                    // outcome for deciding which state to pursue next.
                    if rand::random() {
                        current_state = lhs.unwrap();
                    } else {
                        current_state = rhs.unwrap();
                    }
                }
                End => break,
            }
        }
        output
    }
}

impl Add for NFA {
    type Output = Self;

    /**
     * @brief Takes the current NFA and appends the states
     * of the rhs NFA.
     * @returns Returns the concatenated NFA.
     */
    fn add(self, rhs: Self) -> Self {
        // Assign self to a mutable nfa so we can pop the lhs' End State.
        let mut nfa = self;
        nfa.states.pop();

        // The offset used for adding the rhs' next states.
        let rhs_offset = nfa.states.len();

        let mut nfa_rhs = rhs;

        for state in 0..nfa_rhs.states.len() {
            match &nfa_rhs.states[state] {
                Start(next) => {
                    nfa_rhs.states[state] = Start(Some(next.unwrap() + rhs_offset));
                }
                Match(c, next) => match c {
                    Char::Any => {
                        nfa_rhs.states[state] =
                            Match(Char::Literal('.'), Some(next.unwrap() + rhs_offset));
                    }
                    Char::Literal(character) => {
                        nfa_rhs.states[state] =
                            Match(Char::Literal(*character), Some(next.unwrap() + rhs_offset));
                    }
                },
                Split(lhs, rhs) => {
                    nfa_rhs.states[state] = Split(
                        Some(lhs.unwrap() + rhs_offset),
                        Some(rhs.unwrap() + rhs_offset),
                    );
                }
                End => break,
            }
        }

        // Append the right hand nfa states to original nfa.
        nfa.states.append(&mut nfa_rhs.states);
        nfa
    }
}

// Unit Tests for thegrep's gen, addition overloading, and accept method //
#[cfg(test)]
mod public_api {
    use super::*;

    // These tests ensure the Gen function generates random strings that
    // successfully traverse the NFA.
    #[test]
    fn simple_gen_test() {
        let nfa = NFA::from(&"ab").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"ab").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn simple_gen_any_char() {
        let nfa = NFA::from(&"a..b").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"a..b").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn simple_alternation_gen() {
        let nfa = NFA::from(&"a|b|c").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"a|b|c").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn simple_kleene_gen() {
        let nfa = NFA::from(&"a*").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"a*").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn kleene_plus_gen() {
        let nfa = NFA::from(&"a*b+c*d+").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"a*b+c*d+").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn kleene_plus_with_alternation_gen() {
        let nfa = NFA::from(&"(ab+)|i(cd)+").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"(ab+)|i(cd)+").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn kleene_star_gen() {
        let nfa = NFA::from(&"abcccc*dd*a*a").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"abcccc*dd*a*a").unwrap().accepts(&nfa_gen));
    }

    #[test]
    fn generate_random_regex() {
        let nfa = NFA::from(&"a*(a|b)+|(c|d)*d").unwrap();
        let nfa_gen = nfa.gen();
        assert!(NFA::from(&"a*(a|b)+|(c|d)*d").unwrap().accepts(&nfa_gen));
    }

    // These tests validify of the overloaded add functionality.
    #[test]
    fn simple_addition_overload() {
        let ab = NFA::from(&"ab").unwrap();
        let cd = NFA::from(&"cd").unwrap();
        let abcd = ab + cd;
        assert!(abcd.accepts("abcd"));
        assert!(!abcd.accepts("abcde"));
    }

    #[test]
    fn kleene_addition_overload() {
        let a_star = NFA::from(&"a*").unwrap();
        let c_star = NFA::from(&"c*").unwrap();
        let ac = a_star + c_star;
        assert!(ac.accepts("a"));
        assert!(ac.accepts("c"));
        assert!(ac.accepts("ac"));
        assert!(ac.accepts("aaccc"));
    }

    #[test]
    fn alternation_addition_overload() {
        let ab = NFA::from(&"ab").unwrap();
        let cd = NFA::from(&"c|d").unwrap();
        let ab_c_or_d = ab + cd;
        assert!(ab_c_or_d.accepts("abc"));
        assert!(ab_c_or_d.accepts("abd"));
    }

    #[test]
    fn kleene_alternation_overload() {
        let ab_alt_star = NFA::from(&"(a|b)*").unwrap();
        let cd_alt_star = NFA::from(&"(c|d)*").unwrap();
        let ab_c_or_d = ab_alt_star + cd_alt_star;
        assert!(ab_c_or_d.accepts(""));
        assert!(ab_c_or_d.accepts("abd"));
        assert!(ab_c_or_d.accepts("abbbbbba"));
        assert!(ab_c_or_d.accepts("ccccddd"));
        assert!(ab_c_or_d.accepts("d"));
    }

    #[test]
    fn alternation_overload() {
        let abc = NFA::from(&"a|b|c").unwrap();
        let cd = NFA::from(&"c|d").unwrap();
        let ors = abc + cd;
        assert!(ors.accepts("ac"));
        assert!(ors.accepts("ad"));
        assert!(ors.accepts("bc"));
        assert!(ors.accepts("bd"));
        assert!(ors.accepts("cc"));
        assert!(ors.accepts("cd"));
    }

    #[test]
    fn kleene_overload_panic() {
        let abcd = NFA::from(&"abcd").unwrap();
        let ba_star = NFA::from(&"ba+").unwrap();
        let abcd_ba_star = abcd + ba_star;
        assert!(!abcd_ba_star.accepts("abcda"));
        assert!(!abcd_ba_star.accepts("abcdb"));
    }

    #[test]
    fn alternation_overload_just_for_fun() {
        let alternate_lhs = NFA::from(&"a|b|(c|d)*(e|f|g)+").unwrap();
        let alternate_rhs = NFA::from(&"((h|i)*j+)((k|l|m)|n)").unwrap();
        let alternate_overload = alternate_lhs + alternate_rhs;
        assert!(alternate_overload.accepts("ejk"));
        assert!(alternate_overload.accepts("ejn"));
        assert!(alternate_overload.accepts("ahhhhjl"));
        assert!(alternate_overload.accepts("bhhhhjjm"));
    }

    // Ensure the right-hand side is correct as the topmost operator.
    #[test]
    fn alternation_right_hand_side() {
        let abc = NFA::from(&"abc*").unwrap();
        let c_or_d = NFA::from(&"c|d").unwrap();
        let abc_c_or_d = abc + c_or_d;
        assert!(abc_c_or_d.accepts("abc"));
        assert!(abc_c_or_d.accepts("abccccd"));
        assert!(abc_c_or_d.accepts("abd"));
        assert!(abc_c_or_d.accepts("abcc"));
    }

    #[test]
    fn kleene_plus_overload() {
        let ab = NFA::from(&"ab+").unwrap();
        let d = NFA::from(&"d+").unwrap();
        let abd = ab + d;
        assert!(abd.accepts("abd"));
        assert!(abd.accepts("abbbbd"));
        assert!(abd.accepts("abdddd"));
    }

    // These tests ensure the functionality of the KleenePlus Token.
    #[test]
    fn kleene_plus_test() {
        let nfa = NFA::from(&"a+b+").unwrap();
        assert!(nfa.accepts(&"abb"));
        assert!(nfa.accepts(&"ab"));
        assert!(nfa.accepts(&"aaaaab"));
        assert!(nfa.accepts(&"aaaaaaaabbbbbb"));
    }

    #[test]
    fn kleene_alternation() {
        let nfa = NFA::from(&"(a|b)+(c|d)+").unwrap();
        assert!(nfa.accepts(&"abbc"));
        assert!(nfa.accepts(&"adc"));
        let nfa = NFA::from(&"(a|b)+|(c|d)+").unwrap();
        assert!(nfa.accepts(&"abb"));
        assert!(nfa.accepts(&"dc"));
    }

    #[test]
    fn kleene_plus_closure() {
        let nfa = NFA::from(&"(a*)b+").unwrap();
        assert!(nfa.accepts(&"b"));
        assert!(nfa.accepts(&"ab"));
    }

    #[test]
    #[should_panic]
    fn kleene_plus_early_end() {
        let nfa = NFA::from(&"(ab)+b+").unwrap();
        assert!(nfa.accepts(&"ababab"));
    }

    #[test]
    fn kleene_plus_end() {
        let nfa = NFA::from(&"(ab)+bc*").unwrap();
        assert!(nfa.accepts(&"abababb"));
    }

    #[test]
    fn kleene_plus_correct_alternation() {
        let nfa = NFA::from(&"(abc)|b+|(ab)+").unwrap();
        assert!(nfa.accepts(&"abc"));
        assert!(nfa.accepts(&"b"));
        assert!(nfa.accepts(&"ab"));
        assert!(nfa.accepts(&"abababab"));
    }

    // Tests the functionality of the NFA accepts method.
    #[test]
    fn simple_test() {
        let nfa = NFA::from(&"ab").unwrap();
        assert!(nfa.accepts(&"ab"), "Invalid accepts!");
    }

    // The following tests utilize the kleene star to
    // see if the accepts function handles multiple paths.
    #[test]
    fn test_any_kleene() {
        let nfa = NFA::from(&".*").unwrap();
        assert!(nfa.accepts(&"abccc"), "Invalid accepts!");
    }

    #[test]
    fn test_any_kleene_with_middle() {
        let nfa = NFA::from(&".*ab.*").unwrap();
        assert!(nfa.accepts(&"abc"), "Invalid accepts!");
    }

    #[test]
    fn test_any_kleene_with_middle_and_end() {
        let nfa = NFA::from(&"ab.*ab.*ab").unwrap();
        assert!(nfa.accepts(&"abccccccccccccccabcdab"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn test_fail_any_kleene_with_middle_and_end() {
        let nfa = NFA::from(&"ab.*ab.*ab").unwrap();
        assert!(nfa.accepts(&"abcccccccccccccbcdab"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn test_any_kleene_panic() {
        let nfa = NFA::from(&".*a.*").unwrap();
        assert!(nfa.accepts(&"bbbbbb"), "Invalid accepts!");
    }

    // Ensures that we do not accept an input past the NFA's End State.
    #[test]
    #[should_panic]
    fn simple_overflow() {
        let nfa = NFA::from(&"ab").unwrap();
        assert!(nfa.accepts(&"abc"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn non_matching_string() {
        let nfa = NFA::from(&"hello").unwrap();
        assert!(nfa.accepts(&"abc"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn test_fail_alternation() {
        let nfa = NFA::from(&"x|y").unwrap();
        assert!(nfa.accepts(&"z"), "Invalid accepts!");
    }

    // The following three tests validify that the same input
    // with various NFA structures.
    #[test]
    #[should_panic]
    fn beginning_string() {
        let nfa = NFA::from(&"he").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn ending_string() {
        let nfa = NFA::from(&"lo").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn middle_string() {
        let nfa = NFA::from(&"el").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    // Ensures the validity of any combination of AnyChars and KleeneStars.
    #[test]
    fn regex_closure_ending() {
        let nfa = NFA::from(&"he*").unwrap();
        assert!(nfa.accepts(&"heeeeee"), "Invalid accepts!");
    }

    #[test]
    fn regex_any_char() {
        let nfa = NFA::from(&"he..o").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    fn kleene_ending() {
        let nfa = NFA::from(&"h.*").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
        assert!(nfa.accepts(&"h"), "Invalid accepts!");
    }

    #[test]
    fn test_alternation() {
        let nfa = NFA::from(&"(h|e)e(l|y)lo").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    fn test_alternation_with_any_char() {
        let nfa = NFA::from(&"(h|a)..(l|o).").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    fn smorgasbord_of_regex() {
        let nfa = NFA::from(&"(h|a|e)*.(l)(l)(o|a)").unwrap();
        assert!(nfa.accepts(&"hello"), "Invalid accepts!");
    }

    #[test]
    fn beginning_before_closure() {
        let nfa = NFA::from(&"a.*c").unwrap();
        assert!(nfa.accepts(&"aelloc"), "Invalid accepts!");
    }

    // Ya boy @Hank helped visualize this alternation test case out.
    #[test]
    fn hanks_test() {
        let nfa = NFA::from(&"a|b|c").unwrap();
        assert!(nfa.accepts(&"a"), "Invalid accepts!");
        assert!(nfa.accepts(&"b"), "Invalid accepts!");
        assert!(nfa.accepts(&"c"), "Invalid accepts!");
    }

    #[test]
    #[should_panic]
    fn invalid_closure_concat() {
        let nfa = NFA::from(&"abd*c").unwrap();
        assert!(nfa.accepts(&"abbc"), "Invalid accepts!");
    }
}

/**
 * ===== Internal API =====
 */
type StateId = usize;

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * @brief Recursive implementation for traversing the NFA. This
     * function will preorder traverse the NFA until success or failure.
     * @returns Returns true when the path successfully reaches the
     * End state and otherwise returns false for an incorrect match.
     */
    fn recursive_state(
        &self,
        input_vec: &Vec<char>,
        current_state: usize,
        mut position: usize,
    ) -> bool {
        match &self.states[current_state] {
            Start(next) => {
                // Continues to the next state with the previous position.
                self.recursive_state(&input_vec, next.unwrap(), position)
            }
            Match(c, next) => match c {
                // Checks if there are still chars to match against and continues.
                Char::Any => {
                    if position < input_vec.len() {
                        position += 1;
                        return self.recursive_state(&input_vec, next.unwrap(), position);
                    }
                    false
                }
                // Ensures there are still existing chars then checks if the
                // char at the position is equivalent to the character for
                // the NFA's next state.
                Char::Literal(character) => {
                    if position < input_vec.len() {
                        if input_vec[position] == *character {
                            position += 1;
                            return self.recursive_state(&input_vec, next.unwrap(), position);
                        }
                    }
                    false
                }
            },
            // Preorder traversal of the NFA to see if any sequence in the
            // vector matches the NFA simulation.
            Split(lhs, rhs) => {
                let left = self.recursive_state(&input_vec, lhs.unwrap(), position);

                // If the left branch reaches the end state, there is no need
                // to explore the right hand side.
                if left {
                    return left;
                }

                self.recursive_state(&input_vec, rhs.unwrap(), position)
            }
            // The final ending state checks if there are any
            // remaining characters left to traverse the NFA.
            End => position >= input_vec.len(),
        }
    }

    /**
     * @brief Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => {
                let state = self.add_state(Match(Char::Any, None));
                self.create_fragment(state, vec![state])
            }
            AST::Char(c) => {
                let state = self.add_state(Match(Char::Literal(*c), None));
                self.create_fragment(state, vec![state])
            }
            AST::Catenation(lhs, rhs) => {
                let fragment_lhs = self.gen_fragment(lhs);
                let fragment_rhs = self.gen_fragment(rhs);
                self.join_fragment(&fragment_lhs, fragment_rhs.start);
                self.create_fragment(fragment_lhs.start, fragment_rhs.ends)
            }
            AST::Alternation(lhs, rhs) => {
                let fragment_lhs = self.gen_fragment(lhs);
                let fragment_rhs = self.gen_fragment(rhs);
                let lhs_or_rhs =
                    self.add_state(Split(Some(fragment_lhs.start), Some(fragment_rhs.start)));

                // Connect each of the ends of the fragments.
                let mut lhs_vec = fragment_lhs.ends;
                let rhs_vec = fragment_rhs.ends;
                lhs_vec.extend(rhs_vec);

                self.create_fragment(lhs_or_rhs, lhs_vec)
            }
            AST::Closure(value) => {
                let fragment = self.gen_fragment(value);
                let state = self.add_state(Split(Some(fragment.start), None));
                self.join_fragment(&fragment, state);
                self.create_fragment(state, vec![state])
            }
            AST::OneOrMore(value) => {
                let fragment = self.gen_fragment(value);
                let state = self.add_state(Split(Some(fragment.start), None));
                self.join_fragment(&fragment, state);
                self.create_fragment(fragment.start, vec![state])
            }
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }

    /**
     * @brief Helper function for simply returning a Fragment.
     * @returns Returns a fragment with its state and ends specified
     * as the given arguments.
     */
    fn create_fragment(&self, state: StateId, ends: Vec<usize>) -> Fragment {
        Fragment {
            start: state,
            ends: ends,
        }
    }
}
