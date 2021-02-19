fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let write_result = write_database(key,value);
    match write_result {
        Ok(()) => {
            println!("It worked!");
        }
        Err(the_error) => {
            println!("We got an error {}", the_error);
        }
    }

}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    let contents = format!("{} {}", key, value);
    std::fs::write("kv.db", contents)

}
