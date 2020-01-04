#[doc = "Reader of register FATCON"]
pub type R = crate::R<u32, super::FATCON>;
#[doc = "Writer for register FATCON"]
pub type W = crate::W<u32, super::FATCON>;
#[doc = "Register FATCON `reset()`'s with value 0"]
impl crate::ResetValue for super::FATCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPSEN`"]
pub type FPSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPSEN`"]
pub struct FPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSEN_W<'a> {
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
#[doc = "Reader of field `FATS`"]
pub type FATS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FATS`"]
pub struct FATS_W<'a> {
    w: &'a mut W,
}
impl<'a> FATS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Power Save Enable If CPU clock is slower than 24 MHz, then s/w can enable flash power saving function. 1 = Enable flash power saving 0 = Disable flash power saving"]
    #[inline(always)]
    pub fn fpsen(&self) -> FPSEN_R {
        FPSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Flash Access Time Window Select These bits are used to decide flash sense amplifier active duration. FATS Access Time window (ns) 000 40 001 50 010 60 011 70 100 80 101 90 110 100 111 reserved"]
    #[inline(always)]
    pub fn fats(&self) -> FATS_R {
        FATS_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Power Save Enable If CPU clock is slower than 24 MHz, then s/w can enable flash power saving function. 1 = Enable flash power saving 0 = Disable flash power saving"]
    #[inline(always)]
    pub fn fpsen(&mut self) -> FPSEN_W {
        FPSEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Flash Access Time Window Select These bits are used to decide flash sense amplifier active duration. FATS Access Time window (ns) 000 40 001 50 010 60 011 70 100 80 101 90 110 100 111 reserved"]
    #[inline(always)]
    pub fn fats(&mut self) -> FATS_W {
        FATS_W { w: self }
    }
}
