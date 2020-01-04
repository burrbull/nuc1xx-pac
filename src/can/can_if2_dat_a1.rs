#[doc = "Reader of register CAN_IF2_DAT_A1"]
pub type R = crate::R<u32, super::CAN_IF2_DAT_A1>;
#[doc = "Writer for register CAN_IF2_DAT_A1"]
pub type W = crate::W<u32, super::CAN_IF2_DAT_A1>;
#[doc = "Register CAN_IF2_DAT_A1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF2_DAT_A1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Data0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Data0`"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `Data1`"]
pub type DATA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Data1`"]
pub struct DATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 0 1st data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1 2nd data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0 1st data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1 2nd data byte of a CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W { w: self }
    }
}
