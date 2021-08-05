use is_thirteen::IsThirteen;
use std::io::{stdin, Read};

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    println!("{}", input.thirteen());
    Ok(())
}
