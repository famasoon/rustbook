#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    Ok(f)
}

fn main() {
    let f1_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut buffer: Vec<u8> = vec![];

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, &f1_length);
    println!("{}", text);
}
