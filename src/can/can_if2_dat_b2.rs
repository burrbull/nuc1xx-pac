#[doc = "Reader of register CAN_IF2_DAT_B2"]
pub type R = crate::R<u32, super::CAN_IF2_DAT_B2>;
#[doc = "Writer for register CAN_IF2_DAT_B2"]
pub type W = crate::W<u32, super::CAN_IF2_DAT_B2>;
#[doc = "Register CAN_IF2_DAT_B2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF2_DAT_B2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Data6`"]
pub type DATA6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Data6`"]
pub struct DATA6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `Data7`"]
pub type DATA7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Data7`"]
pub struct DATA7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 6 1st data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 7 2nd data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 6 1st data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 7 2nd data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W { w: self }
    }
}
