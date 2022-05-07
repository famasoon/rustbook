use libactionkv::ActionKV;

const USAGE: &str = "
Usage:
  akv_mem FILE get KEY
  akv_mem FILE delete KEY
  akv_mem FILE insert KEY
  akv_mem FILE update KEY
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE);
    let key = args.get(3).expect(&USAGE);
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        }

        _ => eprintln!("{}", &USAGE)
    }
}