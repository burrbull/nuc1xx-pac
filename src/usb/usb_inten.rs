#[doc = "Reader of register USB_INTEN"]
pub type R = crate::R<u32, super::USB_INTEN>;
#[doc = "Writer for register USB_INTEN"]
pub type W = crate::W<u32, super::USB_INTEN>;
#[doc = "Register USB_INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUS_IE`"]
pub type BUS_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_IE`"]
pub struct BUS_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_IE_W<'a> {
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
#[doc = "Reader of field `USB_IE`"]
pub type USB_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_IE`"]
pub struct USB_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_IE_W<'a> {
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
#[doc = "Reader of field `FLDET_IE`"]
pub type FLDET_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLDET_IE`"]
pub struct FLDET_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLDET_IE_W<'a> {
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
#[doc = "Reader of field `WAKEUP_IE`"]
pub type WAKEUP_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP_IE`"]
pub struct WAKEUP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_IE_W<'a> {
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
#[doc = "Reader of field `WAKEUP_EN`"]
pub type WAKEUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP_EN`"]
pub struct WAKEUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `INNAK_EN`"]
pub struct INNAK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INNAK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1/0: Enable/disable BUS event interrupt."]
    #[inline(always)]
    pub fn bus_ie(&self) -> BUS_IE_R {
        BUS_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1/0: Enable/disable USB event interrupt."]
    #[inline(always)]
    pub fn usb_ie(&self) -> USB_IE_R {
        USB_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1/0: Enable/disable Floating detect Interrupt"]
    #[inline(always)]
    pub fn fldet_ie(&self) -> FLDET_IE_R {
        FLDET_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1/0: Enable/disable Wakeup Interrupt."]
    #[inline(always)]
    pub fn wakeup_ie(&self) -> WAKEUP_IE_R {
        WAKEUP_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1/0: Enable/Disable USB wakeup function"]
    #[inline(always)]
    pub fn wakeup_en(&self) -> WAKEUP_EN_R {
        WAKEUP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1/0: Enable/disable BUS event interrupt."]
    #[inline(always)]
    pub fn bus_ie(&mut self) -> BUS_IE_W {
        BUS_IE_W { w: self }
    }
    #[doc = "Bit 1 - 1/0: Enable/disable USB event interrupt."]
    #[inline(always)]
    pub fn usb_ie(&mut self) -> USB_IE_W {
        USB_IE_W { w: self }
    }
    #[doc = "Bit 2 - 1/0: Enable/disable Floating detect Interrupt"]
    #[inline(always)]
    pub fn fldet_ie(&mut self) -> FLDET_IE_W {
        FLDET_IE_W { w: self }
    }
    #[doc = "Bit 3 - 1/0: Enable/disable Wakeup Interrupt."]
    #[inline(always)]
    pub fn wakeup_ie(&mut self) -> WAKEUP_IE_W {
        WAKEUP_IE_W { w: self }
    }
    #[doc = "Bit 8 - 1/0: Enable/Disable USB wakeup function"]
    #[inline(always)]
    pub fn wakeup_en(&mut self) -> WAKEUP_EN_W {
        WAKEUP_EN_W { w: self }
    }
    #[doc = "Bit 15 - 1 = The NAK status is updated into the endpoint status register, USB_EPSTS, when it is set to 1 and there is NAK response in IN token. It also enable the interrupt event when the device responds NAK after receiving IN token. 0 = The NAK status doesn't be updated into the endpoint status register when it was set to 0. It also disable the interrupt event when device responds NAK after receiving IN token"]
    #[inline(always)]
    pub fn innak_en(&mut self) -> INNAK_EN_W {
        INNAK_EN_W { w: self }
    }
}
