// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use crate::schemes::utils;

/// Downloads a file from a git repository using a raw content URL.
///
/// Supports fetching files from:
/// - GitHub: `git://github.com/owner/repo/branch/...path`
/// - GitLab: `git://gitlab.com/owner/repo/branch/...path`
///
/// The function maps the provided git URL to a raw content URL and performs an HTTP GET request
/// to retrieve the file content as a readable stream.
///
/// # Arguments
/// * `url` - The git URL specifying the repository, branch, and file path.
///
/// # Returns
/// A `Result` containing a boxed readable stream (`Box<dyn Read>`) on success, or an `Error` on failure.
///
/// # References
/// - [Git Protocol v2](https://git-scm.com/docs/protocol-v2)
/// - [Git Web Interface](https://git-scm.com/docs/gitweb)
/// - [GitHub Permanent Links](https://docs.github.com/en/repositories/working-with-files/using-files/getting-permanent-links-to-files)
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html
    let agent = utils::new_agent(true, None);

    let url = map_git_url_to_raw_url(url.as_str())?;

    // See: https://docs.rs/ureq/3.0.12/ureq/struct.Agent.html#method.get
    // See: https://docs.rs/ureq/3.0.12/ureq/struct.RequestBuilder.html#method.call
    utils::fetch(&agent, &url)
}

/// Maps a git URL to a raw content URL for supported git providers.
///
/// Converts URLs like:
/// - GitHub: `git://github.com/owner/repo/branch/...path` → `https://raw.githubusercontent.com/owner/repo/refs/heads/branch/...path`
/// - GitLab: `git://gitlab.com/owner/repo/branch/...path` → `https://gitlab.com/owner/repo/-/raw/branch/...path`
///
/// # Arguments
/// * `url_str` - The git URL to map.
///
/// # Returns
/// A `Result` containing the raw content URL as a `String` on success, or an `Error` if the URL is invalid or the host is unsupported.
fn map_git_url_to_raw_url(url_str: &str) -> Result<String> {
    let Some(path) = url_str.strip_prefix("git://") else {
        return Err(Error::InvalidGitUrl(format!(
            "Invalid git URL format, expected `git://`: {}",
            url_str
        )));
    };

    let components: Vec<&str> = path.split('/').collect();

    let host = components
        .first()
        .ok_or_else(|| Error::InvalidGitUrl(format!("Invalid git URL format: {}", url_str)))?;

    if components.len() < 5 {
        return Err(Error::InvalidGitUrl(format!(
            "Invalid GitHub git URL format (need at least 5 components): {}",
            url_str
        )));
    }

    let owner = components[1];
    let repo = components[2];
    let version = components[3];
    let file_path = components[4..].join("/");

    match *host {
        "github.com" => Ok(format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            owner, repo, version, file_path
        )),
        "gitlab.com" => Ok(format!(
            "https://gitlab.com/{}/{}/-/raw/{}/{}",
            owner, repo, version, file_path
        )),
        _ => Err(Error::InvalidGitUrl(format!(
            "Unsupported git host: {}",
            host
        ))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn url_mapping() {
        assert_eq!(
            map_git_url_to_raw_url(
                "git://github.com/dryrust/gofer.rs/master/lib/gofer/src/schemes/git.rs"
            )
            .unwrap(),
            "https://raw.githubusercontent.com/dryrust/gofer.rs/master/lib/gofer/src/schemes/git.rs"
        );
        assert_eq!(
            map_git_url_to_raw_url(
                "git://github.com/dryrust/gofer.rs/f4ea4a585c009aefd570cefcb6062dc5d579c6ab/VERSION"
            )
            .unwrap(),
            "https://raw.githubusercontent.com/dryrust/gofer.rs/f4ea4a585c009aefd570cefcb6062dc5d579c6ab/VERSION"
        );
        assert_eq!(
            map_git_url_to_raw_url("git://gitlab.com/rust-lang/rust/master/src/README.md").unwrap(),
            "https://gitlab.com/rust-lang/rust/-/raw/master/src/README.md"
        )
    }
}
