#[doc = "Reader of register PDMA_IERx"]
pub type R = crate::R<u32, super::PDMA_IERX>;
#[doc = "Writer for register PDMA_IERx"]
pub type W = crate::W<u32, super::PDMA_IERX>;
#[doc = "Register PDMA_IERx `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PDMA_IERX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `TABORT_IE`"]
pub type TABORT_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TABORT_IE`"]
pub struct TABORT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TABORT_IE_W<'a> {
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
#[doc = "Reader of field `BLKD_IE`"]
pub type BLKD_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKD_IE`"]
pub struct BLKD_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKD_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Enable 0 = Disable target abort interrupt generation during PDMA transfer. 1 = Enable target abort interrupt generation during PDMA transfer."]
    #[inline(always)]
    pub fn tabort_ie(&self) -> TABORT_IE_R {
        TABORT_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDMA Transfer Done Interrupt Enable 0 = Disable interrupt generator during PDMA transfer done. 1 = Enable interrupt generator during PDMA transfer done."]
    #[inline(always)]
    pub fn blkd_ie(&self) -> BLKD_IE_R {
        BLKD_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Enable 0 = Disable target abort interrupt generation during PDMA transfer. 1 = Enable target abort interrupt generation during PDMA transfer."]
    #[inline(always)]
    pub fn tabort_ie(&mut self) -> TABORT_IE_W {
        TABORT_IE_W { w: self }
    }
    #[doc = "Bit 1 - PDMA Transfer Done Interrupt Enable 0 = Disable interrupt generator during PDMA transfer done. 1 = Enable interrupt generator during PDMA transfer done."]
    #[inline(always)]
    pub fn blkd_ie(&mut self) -> BLKD_IE_W {
        BLKD_IE_W { w: self }
    }
}
