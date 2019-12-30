/**
 * thegrep - Tar Heel egrep
 *
 * @file main.rs
 * @brief Starting point for handling command line inputs before handing
 * the specified flag.
 *
 * Author(s): Austin Hale
 * ONYEN(s): haleau
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */
extern crate structopt;

pub mod nfa;
use self::nfa::helpers::nfa_dot;
use self::nfa::NFA;
use std::io;
use structopt::StructOpt;
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;

#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep")]
struct Opt {
    #[structopt(short = "p", long = "parse", help = "Show Parsed AST")]
    parse: bool,

    #[structopt(short = "t", long = "tokens", help = "Show Tokens")]
    tokens: bool,

    #[structopt(short = "d", long = "dot", help = "Show Dot")]
    dot: bool,

    #[structopt(short = "V", long = "version", help = "Prints version information")]
    version: bool,

    #[structopt(
        short = "g",
        long = "gen",
        default_value = "0",
        help = "Generate random strings"
    )]
    gen_amount: i64,

    #[structopt(help = "Regular Expression Pattern")]
    pattern: String,

    #[structopt(help = "FILES")]
    paths: Vec<String>,
}

fn main() {
    let options = Opt::from_args();

    // Prints only when a pattern and version flag are both present.
    if options.version {
        println!("thegrep Version 1.0.0");
    }

    // Continues to evaluate the pattern if exists and
    // otherwise requires a pattern for the input.
    let result = if options.pattern.len() > 0 {
        eval(&options.pattern, &options)
    } else {
        Ok(())
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

/**
 * @brief Ensures the token and/or parse flags are active.
 * @returns Returns a void Ok result on completion.
 */
fn eval(input: &str, options: &Opt) -> io::Result<()> {
    if options.tokens {
        eval_show_tokens(input);
    }

    if options.parse {
        eval_show_parse(input);
    }

    if options.dot {
        eval_show_dot(input);
    }

    // Checks if files are specified in the path.
    if options.gen_amount > 0 {
        eval_gen_random(&options)?;
    } else if options.paths.len() > 0 {
        print_files(&options)?;
    } else {
        print_stdin(&options)?;
    }

    Ok(())
}

/**
 * @brief Retrieves input directly from stdin and
 * continues to print lines accordingly.
 * @returns Returns a void Ok result on completion.
 */
fn print_stdin(options: &Opt) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    print_lines(reader, &options)
}

use std::fs::File;
use std::io::BufRead;

/**
 * @brief Retrieves files specified in the option paths.
 * @returns Returns a void Ok result on completion.
 */
fn print_files(opt: &Opt) -> io::Result<()> {
    for path in opt.paths.iter() {
        // Get a handle on the file by opening it
        let file = File::open(path)?;
        // Read all of its lines
        let reader = io::BufReader::new(file);
        print_lines(reader, &opt)?;
    }
    Ok(())
}

/**
 * @brief Prints a generated string that the NFA accepts
 * from the given regex pattern.
 * @returns Terminates successfully on completion.
 */
fn eval_gen_random(options: &Opt) -> io::Result<()> {
    let nfa = NFA::from(&format!("({})", &options.pattern)).unwrap();
    for _i in 0..options.gen_amount {
        println!("{}", nfa.gen());
    }
    Ok(())
}

/**
 * @brief Unwraps each line to check if the pattern sent to
 * a new NFA accepts the discovered input.
 * @returns Returns a void Ok result on completion.
 */
fn print_lines<R: BufRead>(reader: R, options: &Opt) -> io::Result<()> {
    let nfa = NFA::from(&format!("{}{}{}", ".*", &options.pattern, ".*")).unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        if nfa.accepts(&line) {
            println!("{}", &line);
        }
    }
    Ok(())
}

/**
 * @brief Reads the input and tokenizes the input from
 * the lexemes.
 */
fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    std::process::exit(0);
}

/**
 * @brief Takes the input and prints a parse tree of its relationship
 * with thegrep's grammar. Otherwise, prints the error message to stderr.
 */
fn eval_show_parse(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        }
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    std::process::exit(0);
}

/**
 * @brief Shows the outputted dot graph of the NFA
 * given a regular expression.
 */
fn eval_show_dot(pattern: &str) {
    let nfa = NFA::from(&pattern).unwrap();
    println!("{}", nfa_dot(&nfa));
    std::process::exit(0);
}
