extern crate leval;
use std::io;
use std::io::Write;
use std::env;


fn main() {
    if env::args().count() > 1 {
        let mut expr = String::new();
        for argument in env::args().skip(1) {
            expr = format!("{} {}", expr, argument);
        }

        eval_and_print(expr);
    } else {
        loop {
            print!("Enter an expression: ");
            io::stdout().flush().ok().expect("failed to flush stdout");

            let mut expr = String::new();
            io::stdin().
                read_line(&mut expr).
                expect("failed to read line");
            
            eval_and_print(expr);
        }
    }
}

fn eval_and_print(expr: String) {
    let result = leval::evaluate(expr.trim());

    match result {
        Ok(v) => println!(" = {}\n", v),
        Err(e) => println!("{}\n", e),
    }
}