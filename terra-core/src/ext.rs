use std::io::{self, Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::Color;

pub trait TerraReadExt: io::Read {
    #[inline]
    /// Reads a [`ULEB128`] encoded u32 from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn read_uleb128_u32(&mut self) -> IOResult<u32> {
        Ok(self.read_uleb128_usize()? as u32)
    }

    #[inline]
    /// Reads a [`ULEB128`] encoded u64 from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn read_uleb128_u64(&mut self) -> IOResult<u64> {
        Ok(self.read_uleb128_usize()? as u64)
    }

    #[inline]
    /// Reads a [`ULEB128`] encoded usize from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn read_uleb128_usize(&mut self) -> IOResult<usize> {
        let mut result: usize = 0;
        let mut shift: usize = 0;

        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7f) as usize) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }

        Ok(result)
    }

    #[inline]
    /// Reads a [`ULEB128`] length-prefixed string from the underlying reader. The length is encoded using the [`ULEB128`] format.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn read_lpstring(&mut self) -> IOResult<String> {
        let length = self.read_uleb128_usize()?;

        let mut buf = Vec::new();
        buf.resize(length, 0);

        let mut read = self.read(&mut buf)?;

        loop {
            if read == length {
                break;
            }

            if read > length {
                return Err(IOError::from(IOErrorKind::Other));
            }

            buf[read] = self.read_u8()?;
            read += 1;
        }

        // TODO: Don't unwrap here
        let string = String::from_utf8(buf).unwrap();

        Ok(string)
    }

    #[inline]
    /// Reads a boolean from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_bool(&mut self) -> IOResult<bool> {
        let num = self.read_u8()?;

        Ok(num != 0)
    }

    #[inline]
    /// Reads an RGB color from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_rgb(&mut self) -> IOResult<Color> {
        Ok([self.read_u8()?, self.read_u8()?, self.read_u8()?])
    }
}

impl<R: io::Read + ?Sized> TerraReadExt for R {}

pub trait TerraWriteExt: io::Write {
    #[inline]
    /// Writes a [`ULEB128`] encoded u32 to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn write_uleb128_u32(&mut self, value: u32) -> IOResult<()> {
        self.write_uleb128_usize(value as usize)
    }

    #[inline]
    /// Writes a [`ULEB128`] encoded u64 to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn write_uleb128_u64(&mut self, value: u64) -> IOResult<()> {
        self.write_uleb128_usize(value as usize)
    }

    #[inline]
    /// Writes a [`ULEB128`] encoded usize to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn write_uleb128_usize(&mut self, value: usize) -> IOResult<()> {
        let mut copy = value;

        loop {
            if copy == 0 {
                break;
            }
            let mut byte = (copy & 0x7f) as u8;
            copy >>= 7;
            if copy != 0 {
                byte |= 0x80;
            }
            self.write_u8(byte)?;
        }

        Ok(())
    }

    #[inline]
    /// Writes a [`ULEB128`] length-prefixed string to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    /// [`ULEB128`]: https://wikipedia.org/wiki/LEB128#Unsigned_LEB128
    fn write_lpstring(&mut self, value: &str) -> IOResult<()> {
        self.write_uleb128_usize(value.len())?;

        self.write(value.as_bytes())?;

        Ok(())
    }

    #[inline]
    /// Writes a bool to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_bool(&mut self, value: bool) -> IOResult<()> {
        self.write_u8(match value {
            true => 1,
            false => 0,
        })
    }

    #[inline]
    /// Writes an RGB color to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_rgb(&mut self, value: &Color) -> IOResult<()> {
        self.write_u8(value[0])?;
        self.write_u8(value[1])?;
        self.write_u8(value[2])?;
        Ok(())
    }
}

impl<W: io::Write + ?Sized> TerraWriteExt for W {}
