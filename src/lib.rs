#[derive(Clone)]
#[allow(dead_code)]
pub enum Token {
    Plus,         // +
    Minus,        // -
    Dot,          // .
    Comma,        // ,
    LessThan,     // <
    GreaterThan,  // >
    LeftBracket,  // [
    RightBracket, // ]
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum Instruction {
    Increment,        // +
    Decrement,        // -
    Print,            // .
    Read,             // ,
    DecrementPtr,     // <
    IncrementPtr,     // >
    Loop(Vec<Instruction>),  // [...]
}

#[allow(unused_macros)]
macro_rules! show_help {
    () => {{
        println!("\x1b[0m\x1b[1m\t=+=+=+=+=+= Encephalon Destroyer =+=+=+=+=+=\x1b[0m");
        println!("A BrainFuck interpreter written by \x1b[42m\x1b[37m JhonnyRice \x1b[0m (https://github.com/JhonnyRice)\n");
        info!("Usage", "encephalon-destroyer [OPTIONS] path_to_file\n");
        println!("Flags:\n\t[-h|--help]\t\tShows this text");
        println!("Flags:\n\t[-u|--usage]\t\tShows complete usage");
        println!("\t[-i|--info]\t\tShows information about this program");
        println!("\t[-v|--version]\t\tShows the version of this program\n\x1b[0m");
        println!("\t[-A|--array] NUMBER_OF_CELLS\t\tModifies the array size (default 2048 cells), replacing it with the NUMBER_OF_CELLS specified\n\x1b[0m");
    }};
}

#[allow(unused_macros)]
macro_rules! show_help_and_crash {
    () => {{
        show_help!();
        std::process::exit(0);
    }};
}

#[allow(unused_macros)]
macro_rules! show_usage {
    () => {{
        println!("\x1b[0m\x1b[1m\t=+=+=+=+=+= Encephalon Destroyer =+=+=+=+=+=\x1b[0m");
        println!("A BrainFuck interpreter written by \x1b[42m\x1b[37m JhonnyRice \x1b[0m (https://github.com/JhonnyRice)\n");
        info!("Usage", "");
        println!("\tencephalon-destroyer [PATH_TO_FILE]");
        println!("\tencephalon-destroyer -h | --help");
        println!("\tencephalon-destroyer -u | --usage");
        println!("\tencephalon-destroyer -v | --version");
        println!("\tencephalon-destroyer -i | --info");
        println!("\tencephalon-destroyer -A | --array [NUMBER] [PATH_TO_FILE]");
        println!("\tencephalon-destroyer [PATH_TO_FILE] -A | --array [NUMBER]");
        std::process::exit(0);
    }};
}

#[allow(unused_macros)]
macro_rules! show_info {
    () => {{
        println!("\x1b[0m\x1b[1m\t=+=+=+=+=+= Encephalon Destroyer =+=+=+=+=+=\x1b[0m");
        println!("\x1b[0mEncephalon Destroyer is a BrainFuck interpreter written by \x1b[42m\x1b[37m JhonnyRice \x1b[0m \x1b[32m(https://github.com/JhonnyRice)\x1b[0m as an experiment and an exercise to learn Rust better. It is highly inspired by bf, a Brainfuck interpreter written by \x1b[43m\x1b[30m Alexander Overvoorde \x1b[0m, aka Overv, \x1b[33m(https://github.com/Overv/bf)\x1b[0m. There are quite a few differences between these two projects, from simple QOL features to actually game-changing ones (such as cell wrapping and the ability to customize this one a bit more).\n\nThis project was really fun and it took me only 4 days to make, which now that I am thinking about it is actually a lot. In the future I might add new features or completely revamp the way this interpreter works, maybe adding even a JIT compiler! (OK I don't know if I will add that because JIT compilers are HARD).\nBut who knows what the future will reserve to us. In the meantime, let me have fun!");
        std::process::exit(0);
    }};
}

#[allow(unused_macros)]
macro_rules! show_version {
    () => {{
        println!("\x1b[0m\x1b[1m\t=+=+=+=+=+= Encephalon Destroyer =+=+=+=+=+=\x1b[0m");
        println!("A BrainFuck interpreter written by \x1b[42m\x1b[37m JhonnyRice \x1b[0m (https://github.com/JhonnyRice) ");
        info!("Version", "1.0");
        std::process::exit(0);
    }};
}

#[allow(unused_macros)]
macro_rules! raise_error {
    ($msg:expr) => {{
        print!("\x1b[0m\x1b[47m\x1b[30m");           // Set Background to white and Foreground to black
        print!(" ! ");                               // Set a ! as the symbol
        print!("\x1b[41m\x1b[37m");                  // Set Background to red and Foreground to white
        print!(" Error: \x1b[0m\x1b[31m {} ", $msg); // Display the error
        print!("\x1b[0m\n");                         // Erase all styles and add a newline
        std::process::exit(1)                        // Stop the program with exit code 1
    }};
}

#[allow(unused_macros)]
macro_rules! info {
    ($message_1:expr, $message_2:expr) => {{
        print!("\x1b[47m\x1b[30m");  // Set Background to white and Foreground to black
        print!(" i ");               // Set a 'i' as the symbol (like the 'i' in info)
        print!("\x1b[44m\x1b[37m");  // Set Background to blue and Foreground to white
        print!(" {}: \x1b[0m\x1b[34m {} ", $message_1, $message_2); // Display the info (both the field name (e.g. Usage, Help, Info, Did you know? etc) and the actual text to be shown)
        print!("\x1b[0m\n");         // Erase all styles and add a newline 
    }};
}