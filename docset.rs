extern mod docset;

use docset::Docset;

fn main() {
    let docset = Docset::new(~"../NET Framework.docset/").unwrap();
    let items = docset.query("a");

    for item in items.iter() {
        println!("{} {} {} {}", item.id, item.name, item.category, item.path.display());
    }
}

