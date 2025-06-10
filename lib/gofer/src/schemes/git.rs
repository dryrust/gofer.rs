// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use reqwest::{blocking::ClientBuilder, redirect};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Downloads a file from a git repository.
///
/// Supports:
/// - GitHub: git://github.com/owner/repo/branch/...path
/// - GitLab: git://gitlab.com/owner/repo/branch/...path
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
/// - GitHub: git://github.com/owner/repo/branch/...path -> https://raw.githubusercontent.com/owner/repo/refs/heads/branch/...path
/// - GitLab: git://gitlab.com/owner/repo/branch/...path -> https://gitlab.com/owner/repo/-/raw/branch/...path
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
