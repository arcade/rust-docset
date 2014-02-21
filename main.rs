extern mod sqlite3;

use std::fmt;
use std::os;

struct DocsetItem {
    id: int,
    name: ~str,
    category: ~str,
    path: ~str
}

fn main() {
    let args: ~[~str] = os::args();

    let database = match sqlite3::open(args[1]) {
        Ok(db) => db,
        Err(e) => {
            println(format!("Error opening db"));
            return;
        }
    };

    let sql = format_args!(fmt::format, "SELECT id, name, type, path FROM searchIndex WHERE name LIKE '%{}%'", args[2]);
    let cursor = database.prepare(sql, &None).unwrap();

    loop {
        let result = cursor.step();

        if result == sqlite3::types::SQLITE_DONE {
            break;
        }

        let item = DocsetItem {
            id: cursor.get_int(0),
            name: cursor.get_text(1),
            category: cursor.get_text(2),
            path: cursor.get_text(3)
        };

        //println(format_args!(fmt::format, "{} {} {} {}", item.id, item.name, item.category, item.path));
    }
}

