#![no_std]

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

// See build.py
static RAW_DATA: &str = include_str!("raw_data");

#[derive(Copy, Clone, PartialEq, Debug)]
struct Entry {
    location: u16,
    subtype_len: u8,
    extension_len: u8,
}

impl Entry {
    fn subtype(self) -> &'static str {
        let loc = self.location as usize;
        let len = self.subtype_len as usize;
        &RAW_DATA[loc..loc + len]
    }

    fn extension(self) -> &'static str {
        let loc = self.location as usize + self.subtype_len as usize;
        let len = self.extension_len as usize;
        &RAW_DATA[loc..loc + len]
    }
}

type Table = &'static [Entry];
type Tables = &'static [(&'static str, Table)];

// See build.py
static LOOKUP: Tables = include!("lookup");

fn find_entry(table: Table, subtype: &str) -> Option<Entry> {
    let idx = table
        .binary_search_by_key(&subtype, |entry| entry.subtype())
        .ok()?;
    Some(table[idx])
}

fn find_table(type_: &str) -> Option<Table> {
    Some(LOOKUP.iter().find(|item| item.0 == type_)?.1)
}

fn parse_mimetype(mimetype: &str) -> Option<(&str, &str)> {
    let idx = mimetype.find('/')?;
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
    let (type_, subtype) = parse_mimetype(mimetype.as_ref())?;
    let table = find_table(type_)?;
    let entry = find_entry(table, subtype)?;
    Some(entry.extension())
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    static NOT_FOUND: &[&str] = &[
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
        "\0",
        "\u{00B5}",
        "\u{00B5}\u{00B5}/\u{00B5}\u{00B5}",
        "\u{00B5}\u{00B5}/\u{00B5}\u{00B5}",
        "a\u{00B5}\u{00B5}//\u{00B5}\u{00B5}",
        "application/clr", // Exists in db.json, but without extensions
        "x-conference/nonexistent",
        "application/xcap-error+xml", // Removed v1.47.0
    ];

    #[test]
    fn not_found() {
        for mimetype in NOT_FOUND {
            assert_eq!(mime2ext(*mimetype), None, "Found {:?}", mimetype);
        }
    }

    static FOUND: &[(&str, &str)] = &[
        ("application/octet-stream", "bin"),
        ("image/png", "png"),
        ("application/davmount+xml", "davmount"),
        ("application/andrew-inset", "ez"),
        ("x-conference/x-cooltalk", "ice"),
        ("audio/amr", "amr"),                          // Added v1.46.0
        ("model/vnd.sap.vds", "vds"),                  // Added v1.47.0
        ("application/ecmascript", "es"),              // Changed v1.47.0
        ("application/vnd.mapbox-vector-tile", "mvt"), // Added v1.48.0
    ];

    #[test]
    fn found() {
        for (mimetype, ext) in FOUND {
            assert_eq!(mime2ext(*mimetype), Some(*ext), "Missing {:?}", mimetype);
        }
    }

    /// Make sure every entry can be retrieved (doesn't panic) and that its
    /// contents are unsurprising.
    #[test]
    fn check_entries() {
        for (type_, entries) in LOOKUP {
            assert!(!type_.is_empty());
            assert!(!type_.contains('/'));

            // Required for binary search
            let mut sorted = entries.to_vec();
            sorted.sort_by_key(|entry| entry.subtype());
            assert_eq!(entries, &sorted);

            for entry in *entries {
                let subtype = entry.subtype();
                let ext = entry.extension();
                assert!(!subtype.is_empty());
                assert!(!subtype.contains('/'));
                assert!(!ext.is_empty());
                assert!(!ext.contains('.'));
                assert!(!ext.contains('/'));

                let mimetype = std::format!("{}/{}", type_, subtype);
                assert_eq!(mime2ext(&mimetype), Some(ext));
            }
        }
    }

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Entry>(), 4);
        assert!(RAW_DATA.len() < u16::MAX.into());
    }
}
