use std::env;

mod dllist;
mod hello;

fn main() {
    // In this example, collect convert Args-type iterator into a vector of
    // strings, otherwise, manipulating command-line arguments becomes awkward
    let argv: Vec<String> = env::args().collect();
    // as_str returns a &str object from a String
    match argv[1].as_str() {
        "hello" => hello::hello(argv[2].as_str()),
        "list" => {
            dllist::test_front();
            dllist::test_back();
            dllist::test_iter();
            dllist::test_iter_ref();
        },
        //match needs a &str object
        _ => println!("invalid option"),
    }
}
