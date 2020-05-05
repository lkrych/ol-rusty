fn main() {
    // let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for reference in haystack.iter() {
        let item = *reference;
        
        let result = match item { //match is an expression that returns a value that can be bound to a variable
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result)
        }
    }
}