#[doc = "Reader of register PDMA_BCRx"]
pub type R = crate::R<u32, super::PDMA_BCRX>;
#[doc = "Writer for register PDMA_BCRx"]
pub type W = crate::W<u32, super::PDMA_BCRX>;
#[doc = "Register PDMA_BCRx `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_BCRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMA_BCR`"]
pub type PDMA_BCR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PDMA_BCR`"]
pub struct PDMA_BCR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_BCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDMA Transfer Byte Count Register This field indicates a 16-bit transfer byte count of PDMA.it must be word alignment."]
    #[inline(always)]
    pub fn pdma_bcr(&self) -> PDMA_BCR_R {
        PDMA_BCR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDMA Transfer Byte Count Register This field indicates a 16-bit transfer byte count of PDMA.it must be word alignment."]
    #[inline(always)]
    pub fn pdma_bcr(&mut self) -> PDMA_BCR_W {
        PDMA_BCR_W { w: self }
    }
}
