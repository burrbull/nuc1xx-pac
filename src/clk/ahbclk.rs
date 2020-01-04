#[doc = "Reader of register AHBCLK"]
pub type R = crate::R<u32, super::AHBCLK>;
#[doc = "Writer for register AHBCLK"]
pub type W = crate::W<u32, super::AHBCLK>;
#[doc = "Register AHBCLK `reset()`'s with value 0x0d"]
impl crate::ResetValue for super::AHBCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0d
    }
}
#[doc = "Reader of field `PDMA_EN`"]
pub type PDMA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMA_EN`"]
pub struct PDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_EN_W<'a> {
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
#[doc = "Reader of field `ISP_EN`"]
pub type ISP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISP_EN`"]
pub struct ISP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EBI_EN`"]
pub type EBI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_EN`"]
pub struct EBI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - PDMA Controller Clock Enable Control. 1 = Enable the PDMA engine clock. 0 = Disable the PDMA engine clock."]
    #[inline(always)]
    pub fn pdma_en(&self) -> PDMA_EN_R {
        PDMA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash ISP Controller Clock Enable Control. 1 = Enable the Flash ISP engine clock. 0 = Disable the Flash ISP engine clock."]
    #[inline(always)]
    pub fn isp_en(&self) -> ISP_EN_R {
        ISP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EBI Controller Clock Enable Control (Low Density Only) 1 = Enable the EBI engine clock. 0 = Disable the EBI engine clock."]
    #[inline(always)]
    pub fn ebi_en(&self) -> EBI_EN_R {
        EBI_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PDMA Controller Clock Enable Control. 1 = Enable the PDMA engine clock. 0 = Disable the PDMA engine clock."]
    #[inline(always)]
    pub fn pdma_en(&mut self) -> PDMA_EN_W {
        PDMA_EN_W { w: self }
    }
    #[doc = "Bit 2 - Flash ISP Controller Clock Enable Control. 1 = Enable the Flash ISP engine clock. 0 = Disable the Flash ISP engine clock."]
    #[inline(always)]
    pub fn isp_en(&mut self) -> ISP_EN_W {
        ISP_EN_W { w: self }
    }
    #[doc = "Bit 3 - EBI Controller Clock Enable Control (Low Density Only) 1 = Enable the EBI engine clock. 0 = Disable the EBI engine clock."]
    #[inline(always)]
    pub fn ebi_en(&mut self) -> EBI_EN_W {
        EBI_EN_W { w: self }
    }
}
