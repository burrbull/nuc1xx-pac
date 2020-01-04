#[doc = "Reader of register TLR"]
pub type R = crate::R<u32, super::TLR>;
#[doc = "Writer for register TLR"]
pub type W = crate::W<u32, super::TLR>;
#[doc = "Register TLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `_1SEC`"]
pub type _1SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1SEC`"]
pub struct _1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> _1SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `_10SEC`"]
pub type _10SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_10SEC`"]
pub struct _10SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> _10SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `_1MIN`"]
pub type _1MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1MIN`"]
pub struct _1MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> _1MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `_10MIN`"]
pub type _10MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_10MIN`"]
pub struct _10MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> _10MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `_1HR`"]
pub type _1HR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_1HR`"]
pub struct _1HR_W<'a> {
    w: &'a mut W,
}
impl<'a> _1HR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `_10HR`"]
pub type _10HR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `_10HR`"]
pub struct _10HR_W<'a> {
    w: &'a mut W,
}
impl<'a> _10HR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 1 Sec Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1sec(&self) -> _1SEC_R {
        _1SEC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 10 Sec Time Digit (0~5)"]
    #[inline(always)]
    pub fn _10sec(&self) -> _10SEC_R {
        _10SEC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - 1 Min Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1min(&self) -> _1MIN_R {
        _1MIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 10 Min Time Digit (0~5)"]
    #[inline(always)]
    pub fn _10min(&self) -> _10MIN_R {
        _10MIN_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - 1 Hour Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1hr(&self) -> _1HR_R {
        _1HR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - 10 Hour Time Digit (0~2)"]
    #[inline(always)]
    pub fn _10hr(&self) -> _10HR_R {
        _10HR_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Sec Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1sec(&mut self) -> _1SEC_W {
        _1SEC_W { w: self }
    }
    #[doc = "Bits 4:6 - 10 Sec Time Digit (0~5)"]
    #[inline(always)]
    pub fn _10sec(&mut self) -> _10SEC_W {
        _10SEC_W { w: self }
    }
    #[doc = "Bits 8:11 - 1 Min Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1min(&mut self) -> _1MIN_W {
        _1MIN_W { w: self }
    }
    #[doc = "Bits 12:14 - 10 Min Time Digit (0~5)"]
    #[inline(always)]
    pub fn _10min(&mut self) -> _10MIN_W {
        _10MIN_W { w: self }
    }
    #[doc = "Bits 16:19 - 1 Hour Time Digit (0~9)"]
    #[inline(always)]
    pub fn _1hr(&mut self) -> _1HR_W {
        _1HR_W { w: self }
    }
    #[doc = "Bits 20:21 - 10 Hour Time Digit (0~2)"]
    #[inline(always)]
    pub fn _10hr(&mut self) -> _10HR_W {
        _10HR_W { w: self }
    }
}
