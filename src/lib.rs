use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

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

    let len = mmap.len();

    for i in 0..len {
        mmap[i] = !mmap[i];
    }

    Ok(len as u64)
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
}
