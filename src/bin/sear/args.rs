/// Command line arguments (parsed via gumdrop)
#[derive(Debug, Options)]
pub struct Args {
    /// Input/output archive file
    #[options(short = "f")]
    pub archive: Option<String>,

    /// Change to the given directory before archiving
    #[options(short = "C")]
    pub chdir: Option<String>,

    /// Create a new .sear archive
    #[options(short = "c")]
    pub create: bool,

    /// Extract a .sear archive
    #[options(short = "x")]
    pub extract: bool,

    /// Path to encryption key
    #[options(short = "K", long = "encryption-key")]
    pub encryption_key: Option<String>,

    /// Path to signing key
    #[options(short = "S", long = "signing-key")]
    pub signing_key: Option<String>,

    /// Path to verify key
    #[options(short = "V", long = "verify-key")]
    pub verify_key: Option<String>,

    /// Preserve absolute pathnames
    #[options(short = "P")]
    pub preserve_pathnames: bool,

    /// Preserve file permissions
    #[options(short = "p")]
    pub preserve_permissions: bool,

    /// Verbose mode
    #[options(short = "v")]
    pub verbose: bool,

    /// Files to include in the archive
    #[options(free)]
    pub files: Vec<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            archive: None,
            chdir: None,
            create: false,
            extract: false,
            encryption_key: None,
            signing_key: None,
            verify_key: None,
            preserve_pathnames: false,
            preserve_permissions: false,
            verbose: false,
            files: vec![],
        }
    }
}
