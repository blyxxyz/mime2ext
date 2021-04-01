use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use serde::Deserialize;

#[derive(Deserialize)]
struct DbEntry {
    extensions: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mime-db/db.json");
    let db_text = std::fs::read_to_string("mime-db/db.json")?;
    let db: HashMap<String, DbEntry> = serde_json::from_str(&db_text)?;

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
    let mut lookup_text = String::from("&[");
    for (type_, mut extensions) in by_type {
        extensions.sort_unstable_by_key(|(subtype, _)| *subtype);
        assert!(!type_.contains(|c| c == '"' || c == '\\'));
        lookup_text.push_str(&format!(r#"("{}", &["#, type_));
        for (subtype, extension) in extensions {
            lookup_text.push_str(&format!(
                "Entry {{ location: {loc}, subtype_len: {sub}, extension_len: {ext} }}, ",
                loc = raw_data.len(),
                sub = subtype.len(),
                ext = extension.len(),
            ));
            raw_data.push_str(subtype);
            raw_data.push_str(extension);
        }
        lookup_text.push_str("]), ");
    }
    lookup_text.push(']');

    // Shouldn't matter, but nice to know
    // In theory character boundary checks could be skipped
    assert!(raw_data.is_ascii());

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut opts = OpenOptions::new();
    opts.create(true).write(true).truncate(true);

    let mut data_file = opts.open(out_dir.join("data"))?;
    data_file.write_all(raw_data.as_bytes())?;

    let mut lookup_file = opts.open(out_dir.join("lookup"))?;
    lookup_file.write_all(lookup_text.as_bytes())?;
    Ok(())
}
