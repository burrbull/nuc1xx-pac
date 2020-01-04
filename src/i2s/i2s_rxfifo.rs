#[doc = "Reader of register I2S_RXFIFO"]
pub type R = crate::R<u32, super::I2S_RXFIFO>;
#[doc = "Reader of field `RXFIFO`"]
pub type RXFIFO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO register I2S contains 8 words (8x32 bit) data buffer for data receive. Read this register to get data in FIFO. The remaining data word number is indicated by RX_LEVEL\\[3:0\\]
in I2S_STATUS register."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
