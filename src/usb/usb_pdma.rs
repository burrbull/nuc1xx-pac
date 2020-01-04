#[doc = "Reader of register USB_PDMA"]
pub type R = crate::R<u32, super::USB_PDMA>;
#[doc = "Writer for register USB_PDMA"]
pub type W = crate::W<u32, super::USB_PDMA>;
#[doc = "Register USB_PDMA `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_PDMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMA_RW`"]
pub type PDMA_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMA_RW`"]
pub struct PDMA_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_RW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1 = The USB PDMA read data from USB buffer to memory 0 = The USB PDMA write data from memory to USB buffer"]
    #[inline(always)]
    pub fn pdma_rw(&self) -> PDMA_RW_R {
        PDMA_RW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = The PDMA function in USB is enabled 0 = The PDMA function in USB is disabled This bit will be automatically cleared after PDMA transfer done"]
    #[inline(always)]
    pub fn pdma_en(&self) -> PDMA_EN_R {
        PDMA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 = The USB PDMA read data from USB buffer to memory 0 = The USB PDMA write data from memory to USB buffer"]
    #[inline(always)]
    pub fn pdma_rw(&mut self) -> PDMA_RW_W {
        PDMA_RW_W { w: self }
    }
    #[doc = "Bit 1 - 1 = The PDMA function in USB is enabled 0 = The PDMA function in USB is disabled This bit will be automatically cleared after PDMA transfer done"]
    #[inline(always)]
    pub fn pdma_en(&mut self) -> PDMA_EN_W {
        PDMA_EN_W { w: self }
    }
}
