extern mod docset;

use docset::Docset;
use std::os;

fn main() {
    let args: ~[~str] = os::args();
    let docset = Docset::new(args[1].clone()).unwrap();
    let items = docset.query(args[2].clone());

    for item in items.iter() {
        println!("{} {} {} {}", item.id, item.name, item.category, item.path.display());
    }
}

