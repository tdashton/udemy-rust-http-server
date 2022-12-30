use std::io::Result as IoResult;
use std::io::Write;

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    /*
     * Dynamic vs Static dispatch:
     *
     * passing a trait here indicates to the compiler that an implementation which has that
     * trait will be passed.
     *
     * Prefixing the trait with dyn indicates to the compilier that the binary will look up the
     * proper method at runtime (smaller binary, runtime penalty for looking up correct function)
     *
     * e.g. pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
     */

    /*
     * Prefixing the trait with impl indicates to the compilier that the binary should generate
     * functions for each type it finds that uses this function (bigger binary, no runtime penalty).
     *
     * e.g. pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
     */

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        return write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        );
    }
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}
