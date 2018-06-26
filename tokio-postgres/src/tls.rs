use bytes::{Buf, BufMut};
use futures::{Future, Poll};
use std::error::Error;
use std::io::{self, Read, Write};
use tokio_io::{AsyncRead, AsyncWrite};

use proto;

pub struct Socket(pub(crate) proto::Socket);

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl AsyncRead for Socket {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.0.prepare_uninitialized_buffer(buf)
    }

    fn read_buf<B>(&mut self, buf: &mut B) -> Poll<usize, io::Error>
    where
        B: BufMut,
    {
        self.0.read_buf(buf)
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl AsyncWrite for Socket {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.0.shutdown()
    }

    fn write_buf<B>(&mut self, buf: &mut B) -> Poll<usize, io::Error>
    where
        B: Buf,
    {
        self.0.write_buf(buf)
    }
}

pub trait TlsConnect {
    fn connect(
        &self,
        domain: &str,
        socket: Socket,
    ) -> Box<Future<Item = Box<TlsStream>, Error = Box<Error + Sync + Send>> + Sync + Send>;
}

pub trait TlsStream: 'static + Sync + Send + AsyncRead + AsyncWrite {}

impl TlsStream for proto::Socket {}
