use std::path::Path;

pub(crate) fn pathbuf_to_str(p: &Path) -> String {
    match p.to_str() {
        Some(s) => s.to_string(),
        None => format!("{:?}", p),
    }
}
