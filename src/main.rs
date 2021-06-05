use core::f32;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

use std::collections::HashMap;

use regex::Regex;

// TODO: Testing

fn print_help()
{
    println!("USAGE");
    println!("    word_counter <file_name> <start_line> <end_line>");
    println!("    word_counter -<option>");
    println!("");
    println!("OPTIONS");
    println!("    -h: Displays this help");
}

fn print_error_and_help(e: Error)
{
    println!("ERROR {}\n", e);
    print_help();
}

fn process_arguments(args: Vec<String>) -> Result<Option<(String, usize, usize)>, Error>
{
    let invalid_args_error = Error::new(ErrorKind::InvalidInput,
                                        "at process_arguments: Invalid arguments!");

    match args.len()
    {
        1 => return Ok(None),
        2 => match args[1].as_str()
             {
                 "-h" => return Ok(None),
                 _    => return Err(invalid_args_error)
             },
        4 => (),
        _ => return Err(invalid_args_error)
    }

    let invalid_start_error = Error::new(ErrorKind::InvalidInput,
                                         "at validate_arguments: Invalid start line.");
    let invalid_end_error   = Error::new(ErrorKind::InvalidInput,
                                         "at validate_arguments: Invalid end line.");

    let contents = fs::read_to_string(args[1].as_str())?;

    // TODO: Manage these 2 more gracefully
    let start_line = args[2].parse::<i32>().unwrap_or(-1);
    if start_line < 1 { return Err(invalid_start_error) }

    let end_line   = args[3].parse::<i32>().unwrap_or(-1);
    if end_line < 1 || end_line < start_line { return Err(invalid_end_error) }

    Ok(Some((contents, start_line as usize, end_line as usize)))
}

fn should_skip_this_line(line: &str) -> bool
{
    // Skip comments and defines
    // TODO: skip /**/ too
    match line.get(0..1)
    {
        Some(w) => match w
        {
            "/" | "#" => return true,
            _ => return false
        }
        None => return true // Empty line
    }
}

// Filter out empty words, flow control, basic types and numbers
fn should_skip_this_word(word: &str) -> bool
{
    match word
    {
        "for"       |
        "if"        |
        "else"      |
        "while"     |
        "do"        |
        "switch"    |
        "case"      |
        "continue"  |
        "break"     |
        "auto"      |
        "int"       |
        "float"     |
        "double"    |
        "bool"      |
        "" => true,

        _ => word.parse::<i32>().is_ok()
    }
}

fn parse(contents: String, start: usize, end: usize) -> Result<HashMap<String, usize>, Error>
{
    assert!(start > 0);

    let mut dictionary: HashMap<String, usize>= HashMap::new();

    let mut line_count = 0;
    for line in contents.lines()
    {
        // TODO: I'm pretty sure this could be done in a more elegant way
        line_count += 1;
        if line_count < start { continue; }
        else if line_count > end { break; }

        if should_skip_this_line(line) { continue; }

        // TODO: Skip inline comments

        let re = Regex::new(r"[?!@{}();,.|\-+*/<>=:&^'\[\]\s]\s*").unwrap();
        for word in re.split(line)
        {
            // Don't count numbers or empty words

            if should_skip_this_word(word) { continue; }

            let word_count = match dictionary.get(word)
            {
                Some(v) => v+1,
                None    => 1
            };

            dictionary.insert(word.to_string(), word_count);
        }
    }

    Ok(dictionary)
}

fn sort_and_print_results(dictionary: HashMap<String, usize>)
{
    let mut v: Vec<(String, usize, f32)> = Vec::new();
    let mut total = 0;

    for pair in dictionary
    {
        total += pair.1;
        v.push( (pair.0, pair.1, 0.0) );
    }

    v.sort_by(|a, b| b.1.cmp(&a.1));

    println!("WORD | OCCURRENCES | PERCENTAGE");
    println!("-------------------------------");
    for t in v
    {
        println!("{}: {} ({}%)", t.0, t.1, (t.1 as f32 / total as f32) * 100.0);
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    println!("{:?}", args);

    match process_arguments(args)
    {
        Ok(o) => match o
        {
            Some((contents, start, end)) =>
            {
                match parse(contents, start, end)
                {
                    Ok(d) => sort_and_print_results(d),
                    Err(e) => println!("ERROR {}", e)
                }
            },

            None => print_help() // No arguments
        },

        Err(e) => print_error_and_help(e) // Incorrect set of arguments
    }
}