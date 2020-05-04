fn hello_international() {
    println!("Hello, world!"); // the oldest of dear dear friends.

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    hello_international();
}
