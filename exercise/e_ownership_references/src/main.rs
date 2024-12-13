// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

     let mut material = "mud".to_string();
     println!("This material is just `{}`.", material);
     bedazzle(&mut material);
     println!("Wow! Now the material is `{}`!", material);
}

fn bedazzle (s: &mut String) {
    (*s) = "sparkly".to_string();
}

fn eat (s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

fn change (s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Not plural");
    }
}
