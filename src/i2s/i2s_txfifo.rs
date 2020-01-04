#[doc = "Writer for register I2S_TXFIFO"]
pub type W = crate::W<u32, super::I2S_TXFIFO>;
#[doc = "Register I2S_TXFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TXFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXFIFO`"]
pub struct TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit FIFO register I2S contains 8 words (8x32 bit) data buffer for data transmit. Write data to this register to prepare data for transmit. The remain word number is indicated by TX_LEVEL\\[3:0\\]
in I2S_STATUS."]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TXFIFO_W {
        TXFIFO_W { w: self }
    }
}
