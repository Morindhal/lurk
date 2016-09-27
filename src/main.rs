extern crate clap;

use clap::{Arg, SubCommand};
use clap::App;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main()
{
    let matches = App::new("Lurk")
        .version("1.0")
        .author("Bergman. <Morindhal@gmail.com>")
        .about("Displays text files, optionally filtered by search terms.")
            .arg(Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("v")    /* Currently not used, should be fixed to limit row-lines displayed */
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .subcommand(SubCommand::with_name("search")
                .about("Search module")
                .version("1.0")
                .author("Bergman. <morindhal@gmail.com>")
                .arg(Arg::with_name("searchterm")
                    .multiple(true)
                    .help("Select search-terms to highlight"))
                .arg(Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Shows all lines even when the --search subcommand is used.")))
        .get_matches();
/*    match matches.occurrences_of("v")
    {
        1 => println!("Just a little verbose"),
        2 => println!("Just a little more verbose"),
        3 => println!("Very verbose"),
        4 => println!("Holy MMORPG, that's some verbose shit!!"),
        _ => println!("Maybe verbose, maybe not.")
    }
*/

    let mut searchterm = vec![""];
    let mut search_show_all = true;
    if let Some(searchie) = matches.subcommand_matches("search")
        {
            searchterm = searchie.values_of("searchterm").unwrap().collect();
            search_show_all = searchie.is_present("all");
        }
    
    let from_file = matches.value_of("FILE").unwrap();
    
    let mut buffer = String::new();
    

    if let Err(e) = read_from_file(from_file, &mut buffer, search_show_all, searchterm)
    {
        buffer = String::from(format!("Error reading from file: {}", e) );
    }
    
    println!("\x1B[37m{}\x1B[37m", buffer);
}

fn read_from_file<'a>(from_file : &str, to_string : &'a mut String, all : bool, searchterm : Vec<&str>)
    -> Result<(&'a mut String), io::Error>
{
    let f = try!(File::open(from_file));
    let mut file = BufReader::new(&f);

    let mut read_line = String::new();
    let white:&str = "\x1B[37m";
    let mut keep_line = false;
    while file.read_line(&mut read_line).unwrap() > 0
    {
        for i in 0..searchterm.len()
        {
            if searchterm[i] != "" && match read_line.find(searchterm[i]) { None => false, Some(_) => true }
            {
                read_line = read_line.replace(searchterm[i], &*format!("{}{}{}",get_color(i),searchterm[i],white));
                keep_line = true;
            }
        }
        if keep_line || all
        {
            to_string.push_str(read_line.as_str());
            keep_line = false;
        }
        read_line.clear();
    }

    Ok(to_string)
}

fn get_color(i:usize) -> &'static str
{
    match i
    {
        0=>"\x1B[31m",
        1=>"\x1B[32m",
        2=>"\x1B[33m",
        3=>"\x1B[34m",
        4=>"\x1B[35m",
        5=>"\x1B[36m",
        _=>"\x1B[37m"
    }
}
