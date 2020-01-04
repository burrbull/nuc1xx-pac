#[doc = "Reader of register I2CTOC"]
pub type R = crate::R<u32, super::I2CTOC>;
#[doc = "Writer for register I2CTOC"]
pub type W = crate::W<u32, super::I2CTOC>;
#[doc = "Register I2CTOC `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CTOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIF`"]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
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
#[doc = "Reader of field `DIV4`"]
pub type DIV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV4`"]
pub struct DIV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV4_W<'a> {
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
#[doc = "Reader of field `ENTI`"]
pub type ENTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTI`"]
pub struct ENTI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Time-Out Flag 1 = Time-Out falg is set by H/W. It can interrupt CPU. 0 = S/W can clear the flag."]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time-Out counter input clock is divided by 4 1 = Enable 0 = Disable When Enable, The time-Out period is extend 4 times."]
    #[inline(always)]
    pub fn div4(&self) -> DIV4_R {
        DIV4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-out counter is enabled/disable 1 = Enable 0 = Disable When Enable, the 14 bit time-out counter will start counting when SI is clear. Setting flag SI to high will reset counter and re-start up counting after SI is cleared."]
    #[inline(always)]
    pub fn enti(&self) -> ENTI_R {
        ENTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time-Out Flag 1 = Time-Out falg is set by H/W. It can interrupt CPU. 0 = S/W can clear the flag."]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    #[doc = "Bit 1 - Time-Out counter input clock is divided by 4 1 = Enable 0 = Disable When Enable, The time-Out period is extend 4 times."]
    #[inline(always)]
    pub fn div4(&mut self) -> DIV4_W {
        DIV4_W { w: self }
    }
    #[doc = "Bit 2 - Time-out counter is enabled/disable 1 = Enable 0 = Disable When Enable, the 14 bit time-out counter will start counting when SI is clear. Setting flag SI to high will reset counter and re-start up counting after SI is cleared."]
    #[inline(always)]
    pub fn enti(&mut self) -> ENTI_W {
        ENTI_W { w: self }
    }
}
