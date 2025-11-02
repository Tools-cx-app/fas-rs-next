use std::{
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub struct Analyzer {
    sock_addr: PathBuf,
    connect: Option<UnixStream>,
}

impl Analyzer {
    pub fn new<P>(sock_addr: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            sock_addr: sock_addr.as_ref().to_path_buf(),
            connect: None,
        }
    }

    pub fn connect(&mut self) -> Result<&mut Self> {
        self.connect = Some(UnixStream::connect(self.sock_addr.clone())?);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sock_connect() -> Result<()> {
        Analyzer::new("./test.sock").connect()?;

        Ok(())
    }
}
