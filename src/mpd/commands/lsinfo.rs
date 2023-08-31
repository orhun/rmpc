use super::Song;
use anyhow::Result;

// TODO parsing in general should be redone
#[derive(Debug, Default)]
pub struct LsInfo(pub Vec<FileOrDir>);

#[derive(Debug, PartialEq, Eq)]
pub enum FileOrDir {
    Dir(Dir),
    File(Song),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dir {
    /// this is the full path from mpd root
    pub path: String,
    pub full_path: String,
    pub last_modified: String,
}

impl std::str::FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dir = Dir {
            path: "".to_owned(),
            full_path: "".to_owned(),
            last_modified: "".to_owned(),
        };
        for line in s.lines() {
            match line.split_once(": ") {
                Some(("directory", val)) => {
                    dir.full_path = val.to_owned();
                    dir.path = val.split('/').last().unwrap().to_owned();
                }
                Some(("Last-Modified", val)) => dir.last_modified = val.to_owned(),
                _ => {}
            };
        }
        Ok(dir)
    }
}

impl std::str::FromStr for FileOrDir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("file:") {
            Ok(Self::File(s.parse()?))
        } else if s.starts_with("directory:") {
            Ok(Self::Dir(s.parse()?))
        } else {
            // TODO: playlists are not handled, we should somehow ignore this
            // Listing playlists is deprecated use “listplaylists” instead.
            Err(anyhow::anyhow!(
                "Parsing FilOrDir failed. Playlists are not handled yet."
            ))
        }
    }
}

impl std::str::FromStr for LsInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.lines().fold(Vec::new(), |mut acc, val| {
            if val.starts_with("file:") || val.starts_with("directory:") {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(val);
            acc
        });

        Ok(Self(
            r.iter()
                .map(|v| v.join("\n"))
                .map(|v| v.parse::<FileOrDir>())
                .collect::<Result<Vec<FileOrDir>>>()?,
        ))
    }
}
