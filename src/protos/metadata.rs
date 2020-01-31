//! Metadata for files contained within a `.sear` archive

include!(concat!(env!("OUT_DIR"), "/sear.metadata.rs"));

use super::Entry;

impl From<Vec<Entry>> for Index {
    fn from(entries: Vec<Entry>) -> Index {
        Index { entries }
    }
}
