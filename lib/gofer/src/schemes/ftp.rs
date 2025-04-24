// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use percent_encoding::percent_decode;
use std::borrow::Cow;
use suppaftp::FtpStream;

/// See: https://en.wikipedia.org/wiki/FTP
/// See: https://en.wikipedia.org/wiki/FTPS
pub fn open(url: &Url, _secure: bool) -> Result<Box<dyn Read>> {
    let username = match url.username() {
        "" => "anonymous",
        username => username,
    };

    let password = url
        .password()
        .map(|s| percent_decode(s.as_bytes()).decode_utf8_lossy()) // TODO
        .unwrap_or(Cow::default());

    let mut path: Vec<String> = url
        .path_segments()
        .map(|ss| ss.collect::<Vec<&str>>())
        .unwrap_or_default()
        .into_iter()
        .map(|s| percent_decode(s.as_bytes()).decode_utf8_lossy().to_string()) // TODO
        .collect();

    let basename = path
        .pop()
        .ok_or_else(|| Error::InvalidFtpUrl(url.clone()))?;

    let dirname = path.join("/");

    let addrs = url
        .socket_addrs(|| None)
        .map_err(|_| Error::InvalidFtpUrl(url.clone()))?;

    let mut stream = FtpStream::connect(&*addrs)?;

    stream.login(username, &password)?;

    if dirname != "" {
        stream.cwd(dirname)?;
    }

    let buffer = stream.retr_as_buffer(&basename)?;

    stream.quit()?;

    Ok(Box::new(buffer))
}
