use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::Result;

pub trait Protocol {
    async fn read_bool(&mut self) -> Result<bool>;
    async fn read_byte(&mut self) -> Result<i8>;
    async fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>>;
    async fn read_short(&mut self) -> Result<i16>;
    async fn read_int(&mut self) -> Result<i32>;
    async fn read_long(&mut self) -> Result<i64>;
    async fn read_float(&mut self) -> Result<f32>;
    async fn read_double(&mut self) -> Result<f64>;
    async fn read_string(&mut self) -> Result<String>;

    async fn write_bool(&mut self, val: bool) -> Result<()>;
    async fn write_byte(&mut self, val: i8) -> Result<()>;
    async fn write_bytes(&mut self, val: &[u8]) -> Result<()>;
    async fn write_short(&mut self, val: i16) -> Result<()>;
    async fn write_int(&mut self, val: i32) -> Result<()>;
    async fn write_long(&mut self, val: i64) -> Result<()>;
    async fn write_float(&mut self, val: f32) -> Result<()>;
    async fn write_double(&mut self, val: f64) -> Result<()>;
    async fn write_string(&mut self, val: String) -> Result<()>;
}

impl Protocol for TcpStream {
    async fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_i8().await? != 0)
    }

    async fn read_byte(&mut self) -> Result<i8> {
        Ok(self.read_i8().await?)
    }

    async fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; len];
        self.read_buf(&mut buf).await?;

        Ok(buf)
    }

    async fn read_short(&mut self) -> Result<i16> {
        Ok(self.read_i16().await?)
    }

    async fn read_int(&mut self) -> Result<i32> {
        Ok(self.read_i32().await?)
    }

    async fn read_long(&mut self) -> Result<i64> {
        Ok(self.read_i64().await?)
    }

    async fn read_float(&mut self) -> Result<f32> {
        Ok(self.read_f32().await?)
    }

    async fn read_double(&mut self) -> Result<f64> {
        Ok(self.read_f64().await?)
    }

    async fn read_string(&mut self) -> Result<String> {
        let len = self.read_short().await?;
        let mut buf = vec![0; len.try_into()?];
        self.read_buf(&mut buf).await?;

        Ok(cesu8::from_java_cesu8(&buf)?.to_string())
    }

    async fn write_bool(&mut self, val: bool) -> Result<()> {
        Ok(self.write_i8(i8::from(val)).await?)
    }

    async fn write_bytes(&mut self, val: &[u8]) -> Result<()> {
        self.write_all(val).await?;

        Ok(())
    }

    async fn write_byte(&mut self, val: i8) -> Result<()> {
        Ok(self.write_i8(val).await?)
    }

    async fn write_short(&mut self, val: i16) -> Result<()> {
        Ok(self.write_i16(val).await?)
    }

    async fn write_int(&mut self, val: i32) -> Result<()> {
        Ok(self.write_i32(val).await?)
    }

    async fn write_long(&mut self, val: i64) -> Result<()> {
        Ok(self.write_i64(val).await?)
    }

    async fn write_float(&mut self, val: f32) -> Result<()> {
        Ok(self.write_f32(val).await?)
    }

    async fn write_double(&mut self, val: f64) -> Result<()> {
        Ok(self.write_f64(val).await?)
    }

    async fn write_string(&mut self, val: String) -> Result<()> {
        let to_java = cesu8::to_java_cesu8(val.as_str());

        self.write_short(i16::try_from(to_java.len())?).await?;
        self.write_bytes(&to_java).await?;

        Ok(())
    }
}
