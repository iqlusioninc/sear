//! Support for formatting values in a human-friendly way

/// Format a given number of bytes in a human-friendly way
pub fn byte_size(n: u64) -> String {
    /// KiB
    const KIBIBYTE: f64 = 1024.0;

    /// MiB
    const MEBIBYTE: f64 = 1_048_576.0;

    /// GiB
    const GIBIBYTE: f64 = 1_073_741_824.0;

    if n < KIBIBYTE as u64 {
        // bytes
        if n == 1 {
            return "1 byte".to_owned();
        } else {
            return format!("{} bytes", n);
        }
    }

    let n = n as f64;

    if n < MEBIBYTE {
        // kibibytes
        format!("{} KiB", n / KIBIBYTE)
    } else if n < GIBIBYTE {
        // mebibytes
        format!("{} MiB", n / MEBIBYTE)
    } else {
        // gibibytes
        format!("{} GiB", n / GIBIBYTE)
    }
}
