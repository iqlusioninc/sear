use failure::Error;
use sear::Builder;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use args::Args;
use chdir::Chdir;
use keyring::Keyring;

/// Create a new archive
#[derive(Debug)]
pub struct CreateOp {
    /// Input/output archive file
    pub archive: PathBuf,

    /// Change to the given directory before archiving
    pub chdir: Chdir,

    /// Encryption and signing keys
    pub keyring: Keyring,

    /// Preserve absolute pathnames
    pub preserve_pathnames: bool,

    /// Preserve file permissions
    pub preserve_permissions: bool,

    /// Verbose mode
    pub verbose: bool,

    /// Files to include in the archive
    pub files: Vec<PathBuf>,
}

impl CreateOp {
    /// Initialize a create operation from command-line arguments
    pub fn new(args: Args) -> Result<Self, Error> {
        let archive = match args.archive {
            Some(ref path) => PathBuf::from(path),
            None => bail!("no -f option given"),
        };

        let keyring = Keyring::new(&args);
        let chdir = Chdir::new(args.chdir.map(PathBuf::from));
        let files = args.files.iter().map(PathBuf::from).collect();

        Ok(Self {
            archive,
            chdir,
            keyring,
            files,
            preserve_pathnames: args.preserve_pathnames,
            preserve_permissions: args.preserve_permissions,
            verbose: args.verbose,
        })
    }

    /// Create a new .sear archive
    pub fn perform(&self) -> Result<(), Error> {
        assert!(!self.preserve_pathnames, "-P option unsupported");
        assert!(!self.preserve_permissions, "-p option unsupported");

        let archive = File::create(&self.archive)?;

        // Change to the specified directory if one has been configured.
        // Note this intentionally happens AFTER we have created the output file.
        self.chdir.perform()?;

        let mut builder = Builder::new(archive, self.keyring.encryption_key());

        for path in &self.files {
            let mut input = OpenOptions::new().read(true).open(path)?;

            // TODO: store filename and other metadata
            builder.append(&mut input)?;
        }

        Ok(())
    }
}
