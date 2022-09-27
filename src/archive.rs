use std::{collections::BTreeMap, path::PathBuf};

use flate2::read::GzDecoder;
use lapce_plugin::PLUGIN_RPC;
use std::io::{self, Cursor};
use tar::Archive;
use tar::EntryType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DirOrFile {
    File(PathBuf),
    Dir(PathBuf),
}

pub fn unpack(gz_path: PathBuf, target: PathBuf) -> Result<(), io::Error> {
    let bytes = std::fs::read(gz_path)?;
    let cursor = Cursor::new(bytes);
    let mut archive = Archive::new(GzDecoder::new(cursor));
    archive.set_preserve_mtime(false);
    archive.set_preserve_permissions(false);
    std::fs::create_dir_all(&target)?;
    let mut files = BTreeMap::default();

    let mut outpaths = vec![];
    for (i, file) in archive.entries()?.enumerate() {
        let mut file = file?;
        let file_type = file.header().entry_type();
        let path = file.path()?.to_owned().to_path_buf();
        let outpath = target.join(&format!("{i}.bin"));

        let path = match file_type {
            EntryType::Regular => DirOrFile::File(path),
            EntryType::Directory => DirOrFile::Dir(path),
            e => {
                PLUGIN_RPC.stderr(&format!("Archive contains unsupported file type {e:?}"));
                continue;
            }
        };

        let bytes = match &path {
            DirOrFile::File(_) => {
                let _ = file.unpack(&outpath)?;
                std::fs::read(&outpath)?
            }
            DirOrFile::Dir(_) => Vec::new(),
        };

        files.insert(path, bytes);
        outpaths.push(outpath);
    }

    for dir_or_file in files.keys() {
        if let DirOrFile::Dir(dir) = dir_or_file {
            let path = target.join(dir);
            std::fs::create_dir_all(path)?;
        }
    }

    for (dir_or_file, data) in &files {
        if let DirOrFile::File(file) = dir_or_file {
            let path = target.join(file);
            std::fs::write(path, data)?;
        }
    }

    for path in outpaths {
        if path.is_dir() {
            let _ = std::fs::remove_dir(path);
        } else {
            let _ = std::fs::remove_file(path);
        }
    }

    Ok(())
}
