#[doc = "Reader of register ISPDAT"]
pub type R = crate::R<u32, super::ISPDAT>;
#[doc = "Writer for register ISPDAT"]
pub type W = crate::W<u32, super::ISPDAT>;
#[doc = "Register ISPDAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPDAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPDAT`"]
pub type ISPDAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPDAT`"]
pub struct ISPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Data Write data to this register before ISP program operation Read data from this register after ISP read operation"]
    #[inline(always)]
    pub fn ispdat(&self) -> ISPDAT_R {
        ISPDAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISP Data Write data to this register before ISP program operation Read data from this register after ISP read operation"]
    #[inline(always)]
    pub fn ispdat(&mut self) -> ISPDAT_W {
        ISPDAT_W { w: self }
    }
}
