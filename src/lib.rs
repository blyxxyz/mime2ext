#![no_std]
//! A simple compact crate to look up a file extension for a mime type.
//!
//! This crate embeds part of the [`mime-db`](https://github.com/jshttp/mime-db)
//! database.
//! Its version number tracks that of `mime-db`.
//! `mime2ext` version 0.1.49 corresponds to `mime-db` version 1.49.0.

// This database contains around a thousand entries. At 16 bytes per string
// slice and two strings per entry (mimetype and extension) a naive approach
// would have quite a lot of overhead.
//
// All the strings are instead packed into a single string, without
// delimiters, and we work with offsets into the string.
//
// The total string length is below u16::MAX, and individual string lengths
// are below u8::MAX. Each extension is packed after its mimetype, so a pair
// of strings requires one u16 offset and two u8 lengths, for 4 bytes in total
// (instead of 32).
//
// The entries are sorted by key to support a lookup with `binary_search`.
// They are subdivided by type (the part of the mimetype before the slash)
// to avoid unnecessary repetition. There are only around 10 unique types.
//
// This is likely overengineered, but it was fun to design and seems solid.

// The MSRV is 1.6 (for no_std), so some code looks a little archaic.

// See build.py
static RAW_DATA: &'static str = include_str!("raw_data");

#[derive(Copy, Clone, PartialEq, Debug)]
// (location, subtype_len, extension_len)
struct Entry(u16, u8, u8);

impl Entry {
    // Returns bytes to skip expensive UTF-8 slicing.
    fn subtype(self) -> &'static [u8] {
        let loc = self.0 as usize;
        let len = self.1 as usize;
        &RAW_DATA.as_bytes()[loc..loc + len]
    }

    fn extension(self) -> &'static str {
        let loc = self.0 as usize + self.1 as usize;
        let len = self.2 as usize;
        &RAW_DATA[loc..loc + len]
    }
}

type Table = &'static [Entry];
type Tables = &'static [(&'static str, Table)];

// See build.py
static LOOKUP: Tables = include!("lookup");

fn find_entry(table: Table, subtype: &str) -> Option<Entry> {
    let subtype = subtype.as_bytes();
    match table.binary_search_by(|entry| entry.subtype().cmp(subtype)) {
        Ok(idx) => Some(table[idx]),
        Err(_) => None,
    }
}

fn find_table(type_: &str) -> Option<Table> {
    match LOOKUP.iter().find(|item| item.0 == type_) {
        Some(item) => Some(item.1),
        None => None,
    }
}

fn parse_mimetype(mimetype: &str) -> Option<(&str, &str)> {
    let idx = match mimetype.find('/') {
        Some(idx) => idx,
        None => return None,
    };
    let (type_, mut subtype) = mimetype.split_at(idx);
    subtype = &subtype[1..];
    if let Some(idx) = subtype.find(';') {
        subtype = &subtype[..idx];
    }
    Some((type_, subtype))
}

/// Given a mimetype, pick a suitable file extension.
///
/// # Example
///
/// ```
/// use mime2ext::mime2ext;
///
/// assert_eq!(mime2ext("image/png"), Some("png"));
/// assert_eq!(mime2ext("application/octet-stream"), Some("bin"));
/// assert_eq!(mime2ext("text/html; charset=UTF-8"), Some("html"));
/// assert_eq!(mime2ext("notareal/mimetype"), None);
/// assert_eq!(mime2ext("invalid-mimetype"), None);
/// ```
pub fn mime2ext<S: AsRef<str>>(mimetype: S) -> Option<&'static str> {
    match parse_mimetype(mimetype.as_ref()) {
        Some((type_, subtype)) => match find_table(type_) {
            Some(table) => find_entry(table, subtype).map(Entry::extension),
            None => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate core;
    extern crate std;

    static NOT_FOUND: &'static [&'static str] = &[
        "notareal/mimetype",
        "noslash",
        "application/",
        "application/jpeg",
        "application////",
        "application/octet-stream/",
        "/application/octet-stream",
        "application/aaaaaaa",
        "application/zzzzzzz",
        "aaaaaaaa/jpeg",
        "zzzzzzzz/jpeg",
        "",
        "/",
        "//",
        "/;",
        "a/;",
        "/a;",
        "a/a;",
        ";;",
        ";",
        "\0",
        "\u{00B5}",
        "\u{00B5}\u{00B5}/\u{00B5}\u{00B5}",
        "\u{00B5}\u{00B5}/\u{00B5}\u{00B5}",
        "a\u{00B5}\u{00B5}//\u{00B5}\u{00B5}",
        "application/clr", // Exists in db.json, but without extensions
        "x-conference/nonexistent",
        "text/html ;",                // Bad semicolon position
        "application/xcap-error+xml", // Removed v1.47.0
        "image/hsj2",                 // Removed v1.54.0
    ];

    #[test]
    fn not_found() {
        for mimetype in NOT_FOUND {
            assert_eq!(mime2ext(*mimetype), None);
        }
    }

    static FOUND: &'static [(&'static str, &'static str)] = &[
        ("application/octet-stream", "bin"),
        ("image/png", "png"),
        ("application/davmount+xml", "davmount"),
        ("application/andrew-inset", "ez"),
        ("x-conference/x-cooltalk", "ice"),
        ("text/html; charset=UTF-8", "html"),
        ("text/xml;", "xml"),
        ("audio/amr", "amr"),                          // Added v1.46.0
        ("model/vnd.sap.vds", "vds"),                  // Added v1.47.0
        ("application/ecmascript", "ecma"),            // Changed v1.47.0, changed again v1.53.0
        ("application/vnd.mapbox-vector-tile", "mvt"), // Added v1.48.0
        ("model/step-xml+zip", "stpxz"),               // Added v1.49.0
        ("application/express", "exp"),                // Added v1.50.0
        ("text/vnd.familysearch.gedcom", "ged"),       // Added v1.51.0
        ("image/avci", "avci"),                        // Added v1.52.0
        ("image/jxl", "jxl"),                          // Added v1.53.0
        ("text/markdown", "md"),                       // Changed v1.53.0
        ("application/x-blender", "blend"),            // Added v1.54.0
        ("image/jpeg", "jpg"),                         // Changed v1.54.0
    ];

    #[test]
    fn found() {
        for &(mimetype, ext) in FOUND {
            assert_eq!(mime2ext(mimetype), Some(ext));
        }
    }

    /// Make sure every entry can be retrieved (doesn't panic) and that its
    /// contents are unsurprising.
    #[test]
    fn check_entries() {
        for &(type_, entries) in super::LOOKUP {
            assert!(!type_.is_empty());
            assert!(!type_.contains('/'));

            // Required for binary search
            let mut sorted = entries.to_vec();
            sorted.sort_by(|a, b| a.subtype().cmp(b.subtype()));
            let sorted: &[super::Entry] = &sorted;
            assert_eq!(entries, sorted);

            for entry in entries {
                let subtype = core::str::from_utf8(entry.subtype()).unwrap();
                let ext = entry.extension();
                assert!(!subtype.is_empty());
                assert!(!subtype.contains('/'));
                assert!(!ext.is_empty());
                assert!(!ext.contains('.'));
                assert!(!ext.contains('/'));

                let mimetype = std::string::String::from(type_) + "/" + subtype;
                assert_eq!(mime2ext(&mimetype), Some(ext));
            }
        }
    }

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<super::Entry>(), 4);
        assert!(super::RAW_DATA.len() < std::u16::MAX as usize);
    }
}
