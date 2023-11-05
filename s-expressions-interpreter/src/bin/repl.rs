use s_expressions_interpreter::repl::Repl;

fn main() {
    let mut repl = Repl::interactive(">> ");

    if let Err(e) = repl.mainloop() {
        println!("Error: {}", e);
    }
}