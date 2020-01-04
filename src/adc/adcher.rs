#[doc = "Reader of register ADCHER"]
pub type R = crate::R<u32, super::ADCHER>;
#[doc = "Writer for register ADCHER"]
pub type W = crate::W<u32, super::ADCHER>;
#[doc = "Register ADCHER `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCHER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEN0`"]
pub type CHEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN0`"]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
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
#[doc = "Reader of field `CHEN1`"]
pub type CHEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN1`"]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
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
#[doc = "Reader of field `CHEN2`"]
pub type CHEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN2`"]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
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
#[doc = "Reader of field `CHEN3`"]
pub type CHEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN3`"]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CHEN4`"]
pub type CHEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN4`"]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
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
#[doc = "Reader of field `CHEN5`"]
pub type CHEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN5`"]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CHEN6`"]
pub type CHEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN6`"]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CHEN7`"]
pub type CHEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN7`"]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PRESEL`"]
pub type PRESEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESEL`"]
pub struct PRESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Analog Input Channel 0 Enable 1 = Enable 0 = Disable Channel 0 is the default enable channel if CHEN0~7 are set as 0s."]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Input Channel 1 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog Input Channel 2 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog Input Channel 3 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog Input Channel 4 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog Input Channel 5 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog Input Channel 6 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Input Channel 7 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Analog Input Channel 7 select 00: External analog input 01: Internal bandgap voltage 10: Internal temperature sensor 11: Reserved"]
    #[inline(always)]
    pub fn presel(&self) -> PRESEL_R {
        PRESEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Input Channel 0 Enable 1 = Enable 0 = Disable Channel 0 is the default enable channel if CHEN0~7 are set as 0s."]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - Analog Input Channel 1 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - Analog Input Channel 2 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - Analog Input Channel 3 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - Analog Input Channel 4 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - Analog Input Channel 5 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - Analog Input Channel 6 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - Analog Input Channel 7 Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
    #[doc = "Bits 8:9 - Analog Input Channel 7 select 00: External analog input 01: Internal bandgap voltage 10: Internal temperature sensor 11: Reserved"]
    #[inline(always)]
    pub fn presel(&mut self) -> PRESEL_W {
        PRESEL_W { w: self }
    }
}
