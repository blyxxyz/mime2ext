use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::Write;

use serde::Deserialize;

#[derive(Debug)]
struct Entry {
    location: u16,
    subtype_len: u8,
    extension_len: u8,
}

#[derive(Deserialize)]
struct DBEntry {
    extensions: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mime-db/db.json");
    let db_text = std::fs::read_to_string("mime-db/db.json")?;
    let db: HashMap<String, DBEntry> = serde_json::from_str(&db_text)?;

    let mut by_type = HashMap::<&str, Vec<(&str, &str)>>::new();
    for (mime, info) in &db {
        if let Some(extensions) = &info.extensions {
            let (type_, subtype) = mime.split_at(mime.find('/').unwrap());
            by_type
                .entry(type_)
                .or_default()
                .push((&subtype[1..], &extensions[0]));
        }
    }
    let mut by_type: Vec<_> = by_type.into_iter().collect();
    by_type.sort_unstable_by_key(|(type_, _)| *type_);

    let mut raw_data = String::new();
    let mut lookup = Vec::<(&str, Vec<Entry>)>::new();
    for (type_, mut extensions) in by_type {
        extensions.sort_unstable_by_key(|(subtype, _)| *subtype);
        let mut table = Vec::new();
        for (subtype, extension) in extensions {
            table.push(Entry {
                location: raw_data.len().try_into()?,
                subtype_len: subtype.len().try_into()?,
                extension_len: extension.len().try_into()?,
            });
            raw_data.push_str(subtype);
            raw_data.push_str(extension);
        }
        lookup.push((type_, table));
    }

    // Shouldn't matter, but nice to know
    // In theory character boundary checks could be skipped
    assert!(raw_data.is_ascii());

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut opts = OpenOptions::new();
    opts.create(true).write(true).truncate(true);

    let mut data_file = opts.open(out_dir.join("data"))?;
    data_file.write_all(raw_data.as_bytes())?;

    let lookup_text = format!("{:?}", lookup).replace("[", "&[");
    let mut lookup_file = opts.open(out_dir.join("lookup"))?;
    lookup_file.write_all(lookup_text.as_bytes())?;
    Ok(())
}
