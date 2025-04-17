use std::env;

use uuid::Uuid;

use uuid32::Uuid32;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("Invalid arg: uuid or uuid32 must be provided, or a string containing uuids or uuid32s to be converted")
    }

    let arg = args.join(" ");

    let pieces = arg.split("/");
    let mut result = Vec::new();
    let mut num = 0u128;
    for piece in pieces {
        if piece.len() < 1 {
            result.push("".to_owned());
        } else if let Ok(u) = Uuid::try_parse(&piece) {
            let u32: Uuid32 = u.into();
            num = u.as_u128();
            result.push(u32.to_string());
        } else if let Ok(u32) = Uuid32::try_from(piece) {
            let u: Uuid = u32.into();
            num = u.as_u128();
            result.push(u.to_string());
        } else {
            result.push(piece.to_owned());
        }
    }

    if result.len() != 1 {
        println!("{arg} {}", result.join("/"));
    } else {
        println!("{arg} == {} == {}", result[0], num);
    }
}
