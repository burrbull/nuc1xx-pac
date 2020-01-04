#[doc = "Reader of register PDMA_ISRx"]
pub type R = crate::R<u32, super::PDMA_ISRX>;
#[doc = "Writer for register PDMA_ISRx"]
pub type W = crate::W<u32, super::PDMA_ISRX>;
#[doc = "Register PDMA_ISRx `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_ISRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TABORT_IF`"]
pub type TABORT_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TABORT_IF`"]
pub struct TABORT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> TABORT_IF_W<'a> {
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
#[doc = "Reader of field `BLKD_IF`"]
pub type BLKD_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKD_IF`"]
pub struct BLKD_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKD_IF_W<'a> {
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
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Flag 0 = No bus ERROR response received. 1 = Bus ERROR response received. NOTE: Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn tabort_if(&self) -> TABORT_IF_R {
        TABORT_IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Transfer Done Interrupt Flag This bit indicates that PDMA has finished all transfer. 0 = Not finished yet. 1 = Done. NOTE: Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn blkd_if(&self) -> BLKD_IF_R {
        BLKD_IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Flag 0 = No bus ERROR response received. 1 = Bus ERROR response received. NOTE: Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn tabort_if(&mut self) -> TABORT_IF_W {
        TABORT_IF_W { w: self }
    }
    #[doc = "Bit 1 - Block Transfer Done Interrupt Flag This bit indicates that PDMA has finished all transfer. 0 = Not finished yet. 1 = Done. NOTE: Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn blkd_if(&mut self) -> BLKD_IF_W {
        BLKD_IF_W { w: self }
    }
}
