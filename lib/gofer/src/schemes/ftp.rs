// This is free and unencumbered software released into the public domain.

use crate::{Error, Read, Result, Url};
use percent_encoding::percent_decode;
use std::borrow::Cow;
use suppaftp::FtpStream;

/// See: https://en.wikipedia.org/wiki/FTP
/// See: https://en.wikipedia.org/wiki/FTPS
pub fn open<'a, 'b>(url: &'a Url<'b>, _secure: bool) -> Result<Box<dyn Read>> {
    let authority = url
        .authority()
        .ok_or_else(|| Error::InvalidFtpUrl(url.to_string()))?;

    let username = authority.username().unwrap_or("anonymous");

    let password = authority
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
        .ok_or_else(|| Error::InvalidFtpUrl(url.to_string()))?;

    let dirname = path.join("/");

    let mut stream = FtpStream::connect(&authority)?;

    stream.login(username, password.as_ref())?;

    if dirname != "" {
        stream.cwd(dirname)?;
    }

    let buffer = stream.retr_as_buffer(&basename)?;

    stream.quit()?;

    Ok(Box::new(buffer))
}
