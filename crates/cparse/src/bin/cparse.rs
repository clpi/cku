use pest::Parser;

fn main() {
    cparse::grammar::parse(
        r#"
    let x = 3 + ( 2 * 1 )
    (3 + 1) - 2 * x
    (true and false) or (1 == 2)
    var vis x = "hello"

    # let x = 3
    #:
      let x = 3
    :#


    
    "#,
    );
}
