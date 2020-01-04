#[doc = "Reader of register CLR"]
pub type R = crate::R<u32, super::CLR>;
#[doc = "Writer for register CLR"]
pub type W = crate::W<u32, super::CLR>;
#[doc = "Register CLR `reset()`'s with value 0x0005_0101"]
impl crate::ResetValue for super::CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0005_0101
    }
}
#[doc = "Reader of field `_1DAY`"]
pub type _1DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1DAY`"]
pub struct _1DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> _1DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `_10DAY`"]
pub type _10DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_10DAY`"]
pub struct _10DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> _10DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `_1MON`"]
pub type _1MON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1MON`"]
pub struct _1MON_W<'a> {
    w: &'a mut W,
}
impl<'a> _1MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `_10MON`"]
pub type _10MON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `_10MON`"]
pub struct _10MON_W<'a> {
    w: &'a mut W,
}
impl<'a> _10MON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `_1YEAR`"]
pub type _1YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1YEAR`"]
pub struct _1YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> _1YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `_10YEAR`"]
pub type _10YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_10YEAR`"]
pub struct _10YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> _10YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Day Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1day(&self) -> _1DAY_R {
        _1DAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 10-Day Calendar Digit (0~3)"]
    #[inline(always)]
    pub fn _10day(&self) -> _10DAY_R {
        _10DAY_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 1-Month Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1mon(&self) -> _1MON_R {
        _1MON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 10-Month Calendar Digit (0~1)"]
    #[inline(always)]
    pub fn _10mon(&self) -> _10MON_R {
        _10MON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - 1-Year Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1year(&self) -> _1YEAR_R {
        _1YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 10-Year Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _10year(&self) -> _10YEAR_R {
        _10YEAR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Day Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1day(&mut self) -> _1DAY_W {
        _1DAY_W { w: self }
    }
    #[doc = "Bits 4:5 - 10-Day Calendar Digit (0~3)"]
    #[inline(always)]
    pub fn _10day(&mut self) -> _10DAY_W {
        _10DAY_W { w: self }
    }
    #[doc = "Bits 8:11 - 1-Month Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1mon(&mut self) -> _1MON_W {
        _1MON_W { w: self }
    }
    #[doc = "Bit 12 - 10-Month Calendar Digit (0~1)"]
    #[inline(always)]
    pub fn _10mon(&mut self) -> _10MON_W {
        _10MON_W { w: self }
    }
    #[doc = "Bits 16:19 - 1-Year Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _1year(&mut self) -> _1YEAR_W {
        _1YEAR_W { w: self }
    }
    #[doc = "Bits 20:23 - 10-Year Calendar Digit (0~9)"]
    #[inline(always)]
    pub fn _10year(&mut self) -> _10YEAR_W {
        _10YEAR_W { w: self }
    }
}
