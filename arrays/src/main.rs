fn main() {
    let one = [1, 2, 3];
    let two = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for array in &arrays {
        print!("{:?}: ", array);
        for n in array.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        for i in 0..array.len() {
            sum += array[i];
        }
        println!("\t(Σ{:?} = {})", array, sum);
    }
}
