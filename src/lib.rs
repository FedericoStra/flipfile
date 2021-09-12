use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug, Default)]
pub struct Operations {
    pub flip: bool,
    pub reverse: bool,
    pub swab: bool,
}

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

#[cfg(feature = "memmap")]
pub fn flip_file_mmap(file: &mut File) -> std::io::Result<u64> {
    let mut mmap = unsafe { memmap::MmapMut::map_mut(&file)? };

    for b in mmap.iter_mut() {
        *b = !*b;
    }

    Ok(mmap.len() as u64)
}

#[cfg(feature = "memmap")]
pub fn process_file_mmap(file: &mut File, ops: &Operations) -> std::io::Result<u64> {
    let mut mmap = unsafe { memmap::MmapMut::map_mut(&file)? };

    process_buffer(&mut mmap, ops);

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
