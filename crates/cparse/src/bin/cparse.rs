use pest::Parser;

fn main() {
    cparse::grammar::parse("let x = 3 + ( 2 * 1 )");
}
