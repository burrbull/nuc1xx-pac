#[doc = "Reader of register PDMA_SARx"]
pub type R = crate::R<u32, super::PDMA_SARX>;
#[doc = "Writer for register PDMA_SARx"]
pub type W = crate::W<u32, super::PDMA_SARX>;
#[doc = "Register PDMA_SARx `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_SARX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMA_SAR`"]
pub type PDMA_SAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PDMA_SAR`"]
pub struct PDMA_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PDMA Transfer Source Address Register This field indicates a 32-bit source address of PDMA. Note : The source address must be word alignment"]
    #[inline(always)]
    pub fn pdma_sar(&self) -> PDMA_SAR_R {
        PDMA_SAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDMA Transfer Source Address Register This field indicates a 32-bit source address of PDMA. Note : The source address must be word alignment"]
    #[inline(always)]
    pub fn pdma_sar(&mut self) -> PDMA_SAR_W {
        PDMA_SAR_W { w: self }
    }
}
