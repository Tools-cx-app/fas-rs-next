use std::{
    io::Read,
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result};

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

    pub fn connection(&mut self) -> Result<()> {
        self.connect = Some(UnixStream::connect(self.sock_addr.clone())?);

        Ok(())
    }

    pub fn dump(&mut self, timeout: u64) -> (String, i32) {
        let mut connect = {
            if let Some(connect) = &self.connect {
                connect
                    .try_clone()
                    .context("Failed to clone socket connection")
                    .unwrap()
            } else {
                return (String::new(), 0);
            }
        };
        let mut buffer = String::new();

        connect
            .set_read_timeout(Some(Duration::from_secs(timeout)))
            .context("Failed to set timeout")
            .unwrap();
        connect
            .read_to_string(&mut buffer)
            .context("Failed to read socket")
            .unwrap();

        let mid = buffer.find(":").unwrap();

        {
            let (frametime, pid) = buffer.split_at(mid);
            (frametime.to_string(), pid.parse::<i32>().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sock_connect() -> Result<()> {
        Analyzer::new("./test.sock").connection()?;

        Ok(())
    }
}
