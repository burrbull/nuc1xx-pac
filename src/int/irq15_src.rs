#[doc = "Reader of register IRQ15_SRC"]
pub type R = crate::R<u32, super::IRQ15_SRC>;
#[doc = "Reader of field `INT_SRC`"]
pub type INT_SRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Bit2: 0 Bit1: 0 Bit0: SPI1_INT"]
    #[inline(always)]
    pub fn int_src(&self) -> INT_SRC_R {
        INT_SRC_R::new((self.bits & 0x07) as u8)
    }
}
