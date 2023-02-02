use color_eyre::eyre::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct CkuParser;

pub fn parse<'a>(input: &'a str) {
    let pairs = CkuParser::parse(Rule::FILE, input).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::exprs => parse_expr(inner_pair.into_inner()),
                p => {
                    println!("Rule:    {:?}", p);
                    println!("Span:    {:?}", inner_pair.as_span());
                    println!("Text:    {}", inner_pair.as_str());
                }
            }
        }
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) {
    println!("Exprs:    {}", pairs.as_str());
    for inner_pair in pairs.into_iter() {
        match inner_pair.as_rule() {
            Rule::valexpr => parse_valexpr(inner_pair.into_inner()),
            Rule::asgnexpr => parse_asgnexpr(inner_pair.into_inner()),
            p => {
                println!("Rule:    {:?}", p);
                println!("Span:    {:?}", inner_pair.as_span());
                println!("Text:    {}", inner_pair.as_str());
            }
        }
    }
}

fn parse_asgnexpr(pairs: Pairs<Rule>) {
    println!("Asgnexpr:    {}", pairs.as_str());
    for inner_pair in pairs.into_iter() {
        match inner_pair.as_rule() {
            p => {
                println!("Rule:    {:?}", inner_pair.as_rule());
                println!("Span:    {:?}", inner_pair.as_span());
                println!("Text:    {}", inner_pair.as_str());
            }
        }
    }
}

fn parse_valexpr(pairs: Pairs<Rule>) {
    println!("Valexpr:    {}", pairs.as_str());
    for inner_pair in pairs.into_iter() {
        match inner_pair.as_rule() {
            p => {
                println!("Rule:    {:?}", inner_pair.as_rule());
                println!("Span:    {:?}", inner_pair.as_span());
                println!("Text:    {}", inner_pair.as_str());
            }
        }
    }
}
