use unsophisticated_lang::repl::Repl;

fn main() {
    // Interactive REPL if -i flag is passed
    if std::env::args().any(|x| x == "-i") {
        let mut repl = Repl::interactive(">> ");
        repl.mainloop().unwrap();
    }
    // REPL with file input
    else {
        let filepath = std::env::args().nth(1).unwrap();
        let mut repl = Repl::non_interactive(&filepath);
        repl.mainloop().unwrap();
    }
}
