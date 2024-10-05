use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::Result;

pub trait Protocol {
    async fn read_bool(&mut self) -> Result<bool>;
    async fn read_byte(&mut self) -> Result<i8>;
    async fn read_bytes(&mut self, len: usize) -> Result<Vec<i8>>;
    async fn read_short(&mut self) -> Result<i16>;
    async fn read_int(&mut self) -> Result<i32>;
    async fn read_long(&mut self) -> Result<i64>;
    async fn read_float(&mut self) -> Result<f32>;
    async fn read_double(&mut self) -> Result<f64>;
    async fn read_string(&mut self) -> Result<String>;

    async fn write_bytes(&mut self, val: Vec<u8>) -> Result<()>;
}

impl Protocol for TcpStream {
    async fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_i8().await? != 0)
    }

    async fn read_byte(&mut self) -> Result<i8> {
        Ok(self.read_i8().await?)
    }

    async fn read_bytes(&mut self, len: usize) -> Result<Vec<i8>> {
        let mut buf: Vec<i8> = vec![];

        for _ in 0..len {
            buf.push(self.read_byte().await?);
        }

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
        let mut buf = vec![];

        for _ in 0..len {
            buf.push(self.read_u8().await?);
        }

        Ok(cesu8::from_java_cesu8(&buf)?.to_string())
    }

    async fn write_bytes(&mut self, val: Vec<u8>) -> Result<()> {
        self.write_all(&val[..]).await?;

        Ok(())
    }
}
