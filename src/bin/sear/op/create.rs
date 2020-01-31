//! `sear` operations (crate or extract)

use super::chdir::Chdir;
use crate::{
    command::SearCmd,
    error::{Error, ErrorKind},
    formatter,
    prelude::*,
};
use sear::{protos::Entry, Builder, KeyRing};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    process::exit,
};

/// Create a new archive
#[derive(Debug)]
pub struct CreateOp {
    /// Input/output archive file
    pub archive: PathBuf,

    /// Change to the given directory before archiving
    pub chdir: Chdir,

    /// Files to include in the archive
    pub files: Vec<PathBuf>,

    /// Encryption and signing keys
    pub keyring: KeyRing,

    /// Preserve absolute pathnames
    pub preserve_pathnames: bool,

    /// Preserve file permissions
    pub preserve_permissions: bool,

    /// Enable verbose mode (i.e. print filenames)
    pub verbose: bool,
}

impl CreateOp {
    /// Initialize a create operation from command-line arguments
    pub fn new(args: &SearCmd) -> Result<Self, Error> {
        let archive = match args.archive {
            Some(ref path) => PathBuf::from(path),
            None => fail!(ErrorKind::Argument, "no -f option given"),
        };

        let chdir = Chdir::new(args.chdir.as_ref().map(PathBuf::from));
        let files = args.files.iter().map(PathBuf::from).collect();

        let mut keyring = KeyRing::new();

        if let Some(key_path) = &args.encryption_key {
            keyring.load_symmetric_key(key_path)?;
        }

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

        let symmetric_key = self
            .keyring
            .symmetric_key()
            .unwrap_or_else(|| {
                status_err!("no symmetric key selected (use -K flag)");
                exit(1);
            })
            .clone();

        let archive = File::create(&self.archive)?;

        // Change to the specified directory if one has been configured.
        // Note this intentionally happens AFTER we have created the output file.
        self.chdir.perform()?;

        // TODO(tarcieri): configurable chunk size (default parameter)
        let mut builder = Builder::new(archive, symmetric_key, Default::default())?;

        for path in &self.files {
            self.add_file(&mut builder, path)?;
        }

        builder.finish()?;
        Ok(())
    }

    /// Add a file to the given archive
    fn add_file(&self, builder: &mut Builder<File>, path: &Path) -> Result<(), Error> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        // TODO(tarcieri): better leverage metadata when adding files (e.g. handle directories)
        let metadata = file.metadata()?;
        status_ok!(
            "Adding",
            "{} ({})",
            path.display(),
            formatter::byte_size(metadata.len())
        );

        // TODO(tarcieri): store filename and other metadata
        let entry = Entry::default();
        builder.append(entry, &mut file)?;
        Ok(())
    }
}
