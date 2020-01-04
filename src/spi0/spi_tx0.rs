#[doc = "Writer for register SPI_TX0"]
pub type W = crate::W<u32, super::SPI_TX0>;
#[doc = "Register SPI_TX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_TX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Transmit Register The Data Transmit Registers hold the data to be transmitted in the next transfer. The number of valid bits depend on the transmit bit length field in the CNTRL register. For example, if TX_BIT_LEN is set to 0x08 and the TX_NUM is set to 0x0, the bit TX0\\[7:0\\]
will be transmitted in next transfer. If TX_BIT_LEN is set to 0x00 and TX_NUM is set to 0x1, the core will perform two successive 32-bit transmit/receive using the same setting (the order is TX0\\[31:0\\], TX1\\[31:0\\])."]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
}
