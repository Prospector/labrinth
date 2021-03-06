use super::ids::Base62Id;
use super::teams::Team;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The ID of a specific mod, encoded as base62 for usage in the API
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "Base62Id")]
#[serde(into = "Base62Id")]
pub struct ModId(pub u64);

/// The ID of a specific version of a mod
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "Base62Id")]
#[serde(into = "Base62Id")]
pub struct VersionId(pub u64);

/// A mod returned from the API
#[derive(Serialize, Deserialize)]
pub struct Mod {
    /// The ID of the mod, encoded as a base62 string.
    pub id: ModId,
    // TODO: send partial team structure to reduce requests, but avoid sending
    // unnecessary info
    /// The team of people that has ownership of this mod.
    pub team: Team,

    /// The title or name of the mod.
    pub title: String,
    /// A short description of the mod.
    pub description: String,
    /// The date at which the mod was first published.
    pub published: DateTime<Utc>,

    /// The total number of downloads the mod has had.
    pub downloads: u32,
    /// A list of the categories that the mod is in.
    pub categories: Vec<String>,
    /// A list of ids for versions of the mod.
    pub versions: Vec<VersionId>,

    /// The latest version of the mod.
    pub latest_version: Version,

    /// An optional link to where to submit bugs or issues with the mod.
    pub issues_url: Option<String>,
    /// An optional link to the source code for the mod.
    pub source_url: Option<String>,
    /// An optional link to the mod's wiki page or other relevant information.
    pub wiki_url: Option<String>,
}

/// A specific version of a mod
#[derive(Serialize, Deserialize)]
pub struct Version {
    /// The ID of the version, encoded as a base62 string.
    pub id: VersionId,
    /// The ID of the mod this version is for.
    pub mod_id: ModId,

    /// The name of this version
    pub name: String,
    /// A link to the changelog for this version of the mod.
    pub changelog_url: Option<String>,
    /// The date that this version was published.
    pub date_published: DateTime<Utc>,
    /// The number of downloads this specific version has had.
    pub downloads: u32,
    /// The type of the release - `Alpha`, `Beta`, or `Release`.
    pub version_type: VersionType,

    /// A list of files available for download for this version.
    pub files: Vec<VersionFile>,
    /// A list of mods that this version depends on.
    pub dependencies: Vec<ModId>,
    /// A list of versions of Minecraft that this version of the mod supports.
    pub game_versions: Vec<GameVersion>,
}

/// A single mod file, with a url for the file and the file's hash
#[derive(Serialize, Deserialize)]
pub struct VersionFile {
    /// A list of hashes of the file
    pub hashes: Vec<FileHash>,
    /// A direct link to the file for downloading it.
    pub url: String,
}

/// A hash of a mod's file
#[derive(Serialize, Deserialize)]
pub struct FileHash {
    // TODO: decide specific algorithms
    /// The hashing algorithm used for this hash; could be "md5", "sha1", etc
    pub algorithm: String,
    /// The file hash, using the specified algorithm
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

/// A specific version of Minecraft
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameVersion(pub String);

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub filters: Option<String>,
    pub version: Option<String>,
    pub offset: Option<String>,
    pub index: Option<String>,
}
