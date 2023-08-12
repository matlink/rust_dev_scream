// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use kernel::prelude::*;
use kernel::{chrdev, file, io_buffer};

module! {
    type: RustChrdev,
    name: "rust_scream",
    author: "Matlink",
    description: "Screaming device",
    license: "GPL",
}

struct RustFile;

static ALPHABET: &str = "aAhH";

#[vtable]
impl file::Operations for RustFile {
    fn open(_shared: &(), _file: &file::File) -> Result {
        Ok(())
    }

    fn read(
        _this: (),
        _file: &file::File,
        buf: &mut impl io_buffer::IoBufferWriter,
        _: u64,
    ) -> Result<usize> {
        let total_len = buf.len();
        let mut chunkbuf = [0; 256];

        while !buf.is_empty() {
            let chunk = &mut chunkbuf[0..1];
            kernel::random::getrandom_nonblock(chunk)?;
            let r: usize = <u8 as Into<usize>>::into(chunk[0]) % ALPHABET.len();
            let c = ALPHABET.chars().nth(r).unwrap();
            buf.write_slice(&[c as u8])?;
        }
        Ok(total_len)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust screaming device (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust screaming device (exit)\n");
    }
}
