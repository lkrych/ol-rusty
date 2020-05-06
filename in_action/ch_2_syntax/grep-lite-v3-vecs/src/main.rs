fn main() {
    // PARAMETERS
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    //Initialization
    let mut tags : Vec<usize> = Vec::new();                     // tags will hold line numbers where matches occur
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();        // ctx contains a vector per match to hold that match's context lines

    // Pass 1: find matches
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            
            let v = Vec::with_capacity(2 * context_lines + 1);  // reserves space for n items
            ctx.push(v);
        }
    }

    if tags.len() == 0 {
        return; // nothing was found!
    }

    //Pass 2: grab lines
    for (i, line) in haystack.lines().enumerate() {              // For each tag, at every line, check to see if we are nearby a match. When we are, add that line to the relevant Vec<T>
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines); // usize.saturating_sub is subtraction that returns 0 on integer underflow rather than crashing the program
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);        // copy line into a new String and store that locally for each match
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    // Output
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {                // ref line informs the compiler that we wish to borrow this value, rather than move it.
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
