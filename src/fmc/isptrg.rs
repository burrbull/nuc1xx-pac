#[doc = "Reader of register ISPTRG"]
pub type R = crate::R<u32, super::ISPTRG>;
#[doc = "Writer for register ISPTRG"]
pub type W = crate::W<u32, super::ISPTRG>;
#[doc = "Register ISPTRG `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPTRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPGO`"]
pub type ISPGO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISPGO`"]
pub struct ISPGO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPGO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ISP start trigger Write 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished. 1 = ISP is on going 0 = ISP is operation is finished"]
    #[inline(always)]
    pub fn ispgo(&self) -> ISPGO_R {
        ISPGO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISP start trigger Write 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished. 1 = ISP is on going 0 = ISP is operation is finished"]
    #[inline(always)]
    pub fn ispgo(&mut self) -> ISPGO_W {
        ISPGO_W { w: self }
    }
}
