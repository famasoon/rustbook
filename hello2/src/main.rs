fn greet_world() {
    println!("Hello, world!");
    let japanese = "ハローワールド";
    let english = "Hello, world";

    let regions = [japanese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}