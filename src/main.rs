#![feature(stmt_expr_attributes)]

fn main() {
    #[cfg(test)]
    {
        println!("running tests!");
        ::std::process::exit(0);
    }
    #[cfg(not(test))]
    {
        println!("Hello, world!");
    }
}
