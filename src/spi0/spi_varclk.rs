#[doc = "Reader of register SPI_VARCLK"]
pub type R = crate::R<u32, super::SPI_VARCLK>;
#[doc = "Writer for register SPI_VARCLK"]
pub type W = crate::W<u32, super::SPI_VARCLK>;
#[doc = "Register SPI_VARCLK `reset()`'s with value 0x007f_ff87"]
impl crate::ResetValue for super::SPI_VARCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x007f_ff87
    }
}
#[doc = "Reader of field `VARCLK`"]
pub type VARCLK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VARCLK`"]
pub struct VARCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> VARCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Variable Clock Pattern The value in this field is the frequency patterns of the SPI clock. If the bit patterns of VARCLK are 0, the output frequency of SPICLK is according the value of DIVIDER. If the bit patterns of VARCLK are 1, the output frequency of SPICLK is according the value of DIVIDER2. Refer to register SPI_DIVIDER."]
    #[inline(always)]
    pub fn varclk(&self) -> VARCLK_R {
        VARCLK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Variable Clock Pattern The value in this field is the frequency patterns of the SPI clock. If the bit patterns of VARCLK are 0, the output frequency of SPICLK is according the value of DIVIDER. If the bit patterns of VARCLK are 1, the output frequency of SPICLK is according the value of DIVIDER2. Refer to register SPI_DIVIDER."]
    #[inline(always)]
    pub fn varclk(&mut self) -> VARCLK_W {
        VARCLK_W { w: self }
    }
}
