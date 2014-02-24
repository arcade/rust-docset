#[crate_id="docset#0.1"];
#[crate_type = "lib"];

extern mod sqlite3;

use std::fmt;

use sqlite3::types::SQLITE_DONE;
use sqlite3::Database;

pub struct Docset {
    priv path: Path,
    priv database: Database
}

pub struct DocsetItem {
    id: int,
    name: ~str,
    category: ~str,
    path: Path
}

impl Docset {
    pub fn new(path: ~str) -> Option<Docset> {
        let docsetPath = Path::new(path);
        let databasePath = docsetPath.join("Contents/Resources/docSet.dsidx");

        let database = match sqlite3::open(databasePath.as_str().unwrap()) {
            Ok(db) => db,
            Err(_) => {
                println(format!("Error opening db"));
                return None;
            }
        };

        return Some(Docset {
            path: docsetPath,
            database: database
        });
    }
    pub fn query(&self, query: &str) -> ~[DocsetItem]  {
        let mut items = ~[];

        let sql = format_args!(fmt::format, "\
                SELECT id, name, type, path \
                FROM searchIndex WHERE name LIKE '%{}%' \
                ORDER BY name ASC \
                LIMIT 10", query);
        let cursor = self.database.prepare(sql, &None).unwrap();

        loop {
            match cursor.step() {
                SQLITE_DONE => break,
                _ => {
                    items.push(DocsetItem {
                        id: cursor.get_int(0),
                        name: cursor.get_text(1),
                        category: cursor.get_text(2),
                        path: self.path.join(cursor.get_text(3))
                    });
                }
            }
        }

        return items
    }
}
