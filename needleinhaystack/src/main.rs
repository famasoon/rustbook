fn main() {
    let needle = 0o052;
    let haystack = [1, 1, 2, 5, 15, 42, 52, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
