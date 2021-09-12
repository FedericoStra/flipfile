//! Flip the bytes in multiple files.

#![cfg_attr(doc_cfg, feature(doc_cfg))]

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

/// Operations to apply to every byte.
#[derive(Debug, Default)]
pub struct Operations {
    /// Flip the bytes, i.e. negates each bit.
    pub flip: bool,
    /// Reverse the bytes.
    pub reverse: bool,
    /// Swab the bytes, i.e. swap the first 4 and the last 4 bits.
    pub swab: bool,
}

/// Transform each byte in `buffer` according to the operations specified in `ops`.
pub fn process_buffer(buffer: &mut [u8], ops: &Operations) {
    if ops.flip {
        for b in buffer.iter_mut() {
            *b = !*b;
        }
    }

    if ops.reverse & ops.swab {
        for b in buffer.iter_mut() {
            let mut t = *b;
            t = (t & 0xCC) >> 2 | (t & 0x33) << 2;
            t = (t & 0xAA) >> 1 | (t & 0x55) << 1;
            *b = t;
        }
    } else if ops.reverse {
        for b in buffer.iter_mut() {
            let mut t = *b;
            t = (t & 0xF0) >> 4 | (t & 0x0F) << 4;
            t = (t & 0xCC) >> 2 | (t & 0x33) << 2;
            t = (t & 0xAA) >> 1 | (t & 0x55) << 1;
            *b = t;
        }
    } else if ops.swab {
        for b in buffer.iter_mut() {
            let mut t = *b;
            t = (t & 0xF0) >> 4 | (t & 0x0F) << 4;
            *b = t;
        }
    }
}

/// Transform each byte in `file` according to the operations specified in `ops`.
///
/// The file is read through a buffer, which is then transformed via
/// [`process_buffer`](process_buffer) and written back to the file.
pub fn process_file(file: &mut File, ops: &Operations) -> std::io::Result<u64> {
    log::debug!("ops = {:?}", ops);

    let mut nflipped = 0;
    let mut buffer = [0; 1024 * 256];
    loop {
        let pos = file.stream_position()?;
        let nread = file.read(&mut buffer)?;

        log::debug!("pos = {}, nread = {}", pos, nread);
        debug_assert_eq!(pos + nread as u64, file.stream_position()?);

        if nread == 0 {
            break;
        }

        process_buffer(&mut buffer[0..nread], ops);

        file.seek(SeekFrom::Start(pos))?;

        let nwritten = file.write(&buffer[0..nread])?;

        log::trace!("nwritten = {}", nwritten);
        debug_assert_eq!(nread, nwritten);

        nflipped += nread as u64;
    }
    Ok(nflipped)
}

/// Transform each byte in `file` according to the operations specified in `ops`.
///
/// The file is memory mapped and transformed via [`process_buffer`](process_buffer).
#[cfg(feature = "memmap")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "memmap")))]
pub fn process_file_mmap(file: &mut File, ops: &Operations) -> std::io::Result<u64> {
    let mut mmap = unsafe { memmap::MmapMut::map_mut(&file)? };

    process_buffer(&mut mmap, ops);

    Ok(mmap.len() as u64)
}

#[deprecated = "use [`process_file`](process_file) instead"]
pub fn flip_file(file: &mut File) -> std::io::Result<u64> {
    let mut nflipped = 0;
    let mut buffer = [0; 1024 * 256];
    loop {
        let pos = file.stream_position()?;
        let nread = file.read(&mut buffer)?;

        log::debug!("pos = {}, nread = {}", pos, nread);
        debug_assert_eq!(pos + nread as u64, file.stream_position()?);

        if nread == 0 {
            break;
        }

        for i in 0..nread {
            buffer[i] = !buffer[i];
        }

        file.seek(SeekFrom::Start(pos))?;

        let nwritten = file.write(&buffer[0..nread])?;

        log::trace!("nwritten = {}", nwritten);
        debug_assert_eq!(nread, nwritten);

        nflipped += nread as u64;
    }
    Ok(nflipped)
}

#[deprecated = "use [`process_file_mmap`](process_file_mmap) instead"]
#[cfg(feature = "memmap")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "memmap")))]
pub fn flip_file_mmap(file: &mut File) -> std::io::Result<u64> {
    let mut mmap = unsafe { memmap::MmapMut::map_mut(&file)? };

    for b in mmap.iter_mut() {
        *b = !*b;
    }

    Ok(mmap.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_file() -> std::io::Result<()> {
        let mut file = tempfile::tempfile()?;
        for i in 0..255 {
            file.write(&[i])?;
        }
        file.seek(SeekFrom::Start(0))?;
        flip_file(&mut file)?;
        file.seek(SeekFrom::Start(0))?;
        for i in 0..255 {
            let buf = &mut [0];
            file.read(buf)?;
            assert_eq!(buf[0], !i);
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "memmap")]
    fn test_flip_file_mmap() -> std::io::Result<()> {
        let mut file = tempfile::tempfile()?;
        for i in 0..255 {
            file.write(&[i])?;
        }
        file.seek(SeekFrom::Start(0))?;
        flip_file_mmap(&mut file)?;
        file.seek(SeekFrom::Start(0))?;
        for i in 0..255 {
            let buf = &mut [0];
            file.read(buf)?;
            assert_eq!(buf[0], !i);
        }
        Ok(())
    }
}
