// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use reqwest::{blocking::ClientBuilder, redirect};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Downloads a file from a git repository.
///
/// Supports:
/// - GitHub: git://github.com/owner/repo/tree/branch/...path
/// - GitLab: git://gitlab.com/owner/repo/-/blob/branch/...path
///
/// See: https://git-scm.com/docs/protocol-v2
/// See: https://git-scm.com/docs/gitweb
/// See: https://docs.github.com/en/repositories/working-with-files/using-files/getting-permanent-links-to-files
pub fn open<'a, 'b>(url: &'a Url<'b>) -> Result<Box<dyn Read>> {
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html
    let client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .redirect(redirect::Policy::default())
        .https_only(true);

    let url = map_git_url_to_raw_url(url.as_str())?;

    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html#method.get
    // See: https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
    let response = client.build()?.get(url).send()?;

    Ok(Box::new(response))
}

/// Maps a git URL to a raw content URL for supported git providers.
/// - GitHub: git://github.com/owner/repo/tree/branch/...path -> https://raw.githubusercontent.com/owner/repo/refs/heads/branch/...path
/// - GitLab: git://gitlab.com/owner/repo/-/blob/branch/...path -> https://gitlab.com/owner/repo/-/raw/branch/...path
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

    match *host {
        "github.com" => {
            // GitHub format: host/owner/repo/tree/branch/...path
            if components.len() < 6 {
                return Err(Error::InvalidGitUrl(format!(
                    "Invalid GitHub git URL format (need at least 6 components): {}",
                    url_str
                )));
            }

            let owner = components[1];
            let repo = components[2];
            let blob = components[3];

            if blob != "blob" {
                return Err(Error::InvalidGitUrl(format!(
                    "Expected 'blob' after repo name for GitHub, got: {}",
                    blob
                )));
            }

            let branch = components[4];
            let file_path = components[5..].join("/");

            Ok(format!(
                "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/{}",
                owner, repo, branch, file_path
            ))
        }
        "gitlab.com" => {
            // GitLab format: host/owner/repo/-/blob/branch/...path
            if components.len() < 7 {
                return Err(Error::InvalidGitUrl(format!(
                    "Invalid GitLab git URL format (need at least 7 components): {}",
                    url_str
                )));
            }

            let owner = components[1];
            let repo = components[2];
            let dash = components[3];

            if dash != "-" {
                return Err(Error::InvalidGitUrl(format!(
                    "Expected '-' after repo name for GitLab, got: {}",
                    dash
                )));
            }

            let blob = components[4];

            if blob != "blob" {
                return Err(Error::InvalidGitUrl(format!(
                    "Expected 'blob' after '-' for GitLab, got: {}",
                    blob
                )));
            }

            let branch = components[5];
            let file_path = components[6..].join("/");

            Ok(format!(
                "https://gitlab.com/{}/{}/-/raw/{}/{}",
                owner, repo, branch, file_path
            ))
        }
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
                "git://github.com/dryrust/gofer.rs/blob/master/lib/gofer/src/schemes/git.rs"
            )
            .unwrap(),
            "https://raw.githubusercontent.com/dryrust/gofer.rs/refs/heads/master/lib/gofer/src/schemes/git.rs"
        );
        assert_eq!(
            map_git_url_to_raw_url("git://gitlab.com/rust-lang/rust/-/blob/master/src/README.md")
                .unwrap(),
            "https://gitlab.com/rust-lang/rust/-/raw/master/src/README.md"
        )
    }
}
