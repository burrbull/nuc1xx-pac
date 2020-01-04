#[doc = "Reader of register ISPADR"]
pub type R = crate::R<u32, super::ISPADR>;
#[doc = "Writer for register ISPADR"]
pub type W = crate::W<u32, super::ISPADR>;
#[doc = "Register ISPADR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPADR`"]
pub type ISPADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPADR`"]
pub struct ISPADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISP Address NuMicro(TM) NUC100 Series equips with a maximum 32Kx32 embedded flash, it supports word program only. ISPARD\\[1:0\\]
must be kept 00b for ISP operation."]
    #[inline(always)]
    pub fn ispadr(&self) -> ISPADR_R {
        ISPADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISP Address NuMicro(TM) NUC100 Series equips with a maximum 32Kx32 embedded flash, it supports word program only. ISPARD\\[1:0\\]
must be kept 00b for ISP operation."]
    #[inline(always)]
    pub fn ispadr(&mut self) -> ISPADR_W {
        ISPADR_W { w: self }
    }
}
