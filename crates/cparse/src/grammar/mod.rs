use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use color_eyre::eyre::Result;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct CkuParser;

pub fn parse<'a>(input: &'a str) {
    let pairs = CkuParser::parse(Rule::FILE, input)
        .unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::exprs => println!("Exprs:    {}", inner_pair.as_str()),
                p => { 
                    println!("Rule:    {:?}", p);
                    println!("Span:    {:?}", inner_pair.as_span());
                    println!("Text:    {}", inner_pair.as_str());
                },
            }

        }
    }


}
