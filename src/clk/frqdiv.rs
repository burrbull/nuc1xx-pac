#[doc = "Reader of register FRQDIV"]
pub type R = crate::R<u32, super::FRQDIV>;
#[doc = "Writer for register FRQDIV"]
pub type W = crate::W<u32, super::FRQDIV>;
#[doc = "Register FRQDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::FRQDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSEL`"]
pub type FSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSEL`"]
pub struct FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DIVIDER_EN`"]
pub type DIVIDER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVIDER_EN`"]
pub struct DIVIDER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Divider Output Frequency Selection Bits The formula of output frequency is Fout = Fin/(2^(N+1)), Fin is the input clock frequency. Fout is the frequency of divider output clock. N is the 4-bit value of FSEL\\[3:0\\]."]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Frequency Divider Enable Bit 0 = Disable Frequency Divider 1 = Enable Frequency Divider"]
    #[inline(always)]
    pub fn divider_en(&self) -> DIVIDER_EN_R {
        DIVIDER_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Divider Output Frequency Selection Bits The formula of output frequency is Fout = Fin/(2^(N+1)), Fin is the input clock frequency. Fout is the frequency of divider output clock. N is the 4-bit value of FSEL\\[3:0\\]."]
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W {
        FSEL_W { w: self }
    }
    #[doc = "Bit 4 - Frequency Divider Enable Bit 0 = Disable Frequency Divider 1 = Enable Frequency Divider"]
    #[inline(always)]
    pub fn divider_en(&mut self) -> DIVIDER_EN_W {
        DIVIDER_EN_W { w: self }
    }
}
