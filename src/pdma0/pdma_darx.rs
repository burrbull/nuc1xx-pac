#[doc = "Reader of register PDMA_DARx"]
pub type R = crate::R<u32, super::PDMA_DARX>;
#[doc = "Writer for register PDMA_DARx"]
pub type W = crate::W<u32, super::PDMA_DARX>;
#[doc = "Register PDMA_DARx `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_DARX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMA_DAR`"]
pub type PDMA_DAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PDMA_DAR`"]
pub struct PDMA_DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_DAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PDMA Transfer Destination Address Register This field indicates a 32-bit destination address of PDMA. Note : The destination address must be word alignment"]
    #[inline(always)]
    pub fn pdma_dar(&self) -> PDMA_DAR_R {
        PDMA_DAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDMA Transfer Destination Address Register This field indicates a 32-bit destination address of PDMA. Note : The destination address must be word alignment"]
    #[inline(always)]
    pub fn pdma_dar(&mut self) -> PDMA_DAR_W {
        PDMA_DAR_W { w: self }
    }
}
