use unsophisticated_lang::repl::Repl;

fn main() {
    let mut repl = Repl::interactive(">> ");

    if let Err(e) = repl.mainloop() {
        println!("Hey: {}", e);
    }
}