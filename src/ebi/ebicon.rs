#[doc = "Reader of register EBICON"]
pub type R = crate::R<u32, super::EBICON>;
#[doc = "Writer for register EBICON"]
pub type W = crate::W<u32, super::EBICON>;
#[doc = "Register EBICON `reset()`'s with value 0"]
impl crate::ResetValue for super::EBICON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ExtEN`"]
pub type EXTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ExtEN`"]
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ExtBW16`"]
pub type EXTBW16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ExtBW16`"]
pub struct EXTBW16_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTBW16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCLKDIV`"]
pub type MCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCLKDIV`"]
pub struct MCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ExttALE`"]
pub type EXTTALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ExttALE`"]
pub struct EXTTALE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EBI Enable This bit is the functional enable bit for EBI. 1: EBI function is enabled 0: EBI function is disabled"]
    #[inline(always)]
    pub fn ext_en(&self) -> EXTEN_R {
        EXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EBI data width 16 bit This bit defines if the data bus is 8-bit or 16-bit. 1: EBI data width is 16 bit 0: EBI data width is 8 bit"]
    #[inline(always)]
    pub fn ext_bw16(&self) -> EXTBW16_R {
        EXTBW16_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - External Output Clock Divider The frequency of EBI output clock is controlled by MCLKDIV as follows table: MCLKDIV Output clock (MCLK) 000 HCLK/1 001 HCLK/2 010 HCLK/4 011 HCLK/8 100 HCLK/16 101 HCLK/32 11X default Notice: Default value of output clock is HCLK/1"]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Expand Time of ALE The ALE width (tALE) to latch the address can be controlled by ExttALE. tALE = (ExttALE+1)*MCLK"]
    #[inline(always)]
    pub fn extt_ale(&self) -> EXTTALE_R {
        EXTTALE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Enable This bit is the functional enable bit for EBI. 1: EBI function is enabled 0: EBI function is disabled"]
    #[inline(always)]
    pub fn ext_en(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    #[doc = "Bit 1 - EBI data width 16 bit This bit defines if the data bus is 8-bit or 16-bit. 1: EBI data width is 16 bit 0: EBI data width is 8 bit"]
    #[inline(always)]
    pub fn ext_bw16(&mut self) -> EXTBW16_W {
        EXTBW16_W { w: self }
    }
    #[doc = "Bits 8:10 - External Output Clock Divider The frequency of EBI output clock is controlled by MCLKDIV as follows table: MCLKDIV Output clock (MCLK) 000 HCLK/1 001 HCLK/2 010 HCLK/4 011 HCLK/8 100 HCLK/16 101 HCLK/32 11X default Notice: Default value of output clock is HCLK/1"]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W {
        MCLKDIV_W { w: self }
    }
    #[doc = "Bits 16:18 - Expand Time of ALE The ALE width (tALE) to latch the address can be controlled by ExttALE. tALE = (ExttALE+1)*MCLK"]
    #[inline(always)]
    pub fn extt_ale(&mut self) -> EXTTALE_W {
        EXTTALE_W { w: self }
    }
}
