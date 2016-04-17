extern crate openssl;

use std::io;
use std::fmt;
use self::openssl::ssl::{self, SslContext, SslMethod, SSL_VERIFY_PEER};
use self::openssl::ssl::error::SslError;

pub struct Error(SslError);

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error(err)
    }
}

pub struct ClientBuilder(SslContext);

impl ClientBuilder {
    pub fn new() -> Result<ClientBuilder, Error> {
        let mut ctx = try!(SslContext::new(SslMethod::Sslv23));
        ctx.set_verify(SSL_VERIFY_PEER, None);
        try!(ctx.set_default_verify_paths());
        Ok(ClientBuilder(ctx))
    }

    // FIXME hostname verification
    pub fn handshake<S>(&mut self, _domain: &str, stream: S) -> Result<SslStream<S>, Error>
        where S: io::Read + io::Write
    {
        let s = try!(ssl::SslStream::connect(&self.0, stream));
        Ok(SslStream(s))
    }
}

pub struct SslStream<S>(ssl::SslStream<S>);

impl<S> SslStream<S> {
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S: io::Read + io::Write> io::Read for SslStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<S: io::Read + io::Write> io::Write for SslStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
