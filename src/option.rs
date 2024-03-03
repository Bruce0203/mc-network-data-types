use crate::encoding::{Encoder, Decoder};

pub trait OptionRead {
    fn read_option<D: Decoder>(&mut self) -> std::io::Result<D>;
}

pub trait OptionWrite {
    fn write_option<E: Encoder>(&mut self, value: &E) -> std::io::Result<()>;
}

impl<R: std::io::Read> OptionRead for R {
    fn read_option<D: Decoder>(&mut self) -> std::io::Result<D> {
        Ok(D::decode_from_read(self)?)
    }
}

impl<W: std::io::Write> OptionWrite for W {
    fn write_option<E: Encoder>(&mut self, value: &E) -> std::io::Result<()> {
        value.encode_to_write(self)?;
        Ok(())
    }
}
