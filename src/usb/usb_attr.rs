#[doc = "Reader of register USB_ATTR"]
pub type R = crate::R<u32, super::USB_ATTR>;
#[doc = "Writer for register USB_ATTR"]
pub type W = crate::W<u32, super::USB_ATTR>;
#[doc = "Register USB_ATTR `reset()`'s with value 0x40"]
impl crate::ResetValue for super::USB_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHY_EN`"]
pub type PHY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHY_EN`"]
pub struct PHY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RWAKEUP`"]
pub type RWAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWAKEUP`"]
pub struct RWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `USB_EN`"]
pub type USB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_EN`"]
pub struct USB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DPPU_EN`"]
pub type DPPU_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPPU_EN`"]
pub struct DPPU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPU_EN_W<'a> {
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
#[doc = "Reader of field `PWRDN`"]
pub type PWRDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDN`"]
pub struct PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BYTEM`"]
pub type BYTEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEM`"]
pub struct BYTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1: Bus reset when SE0(single-ended 0) more than 2.5uS. 0: Bus no reset."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: Bus idle more than 3mS, either cable is plugged off or host is sleeping. 0: Bus no suspend."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Resume from suspension 0: No bus resume."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: No response more than 18 bits time 0: No time out."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable PHY transceiver function. 0: Disable PHY transceiver function."]
    #[inline(always)]
    pub fn phy_en(&self) -> PHY_EN_R {
        PHY_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: Force USB bus to K state, used for remote wake-up. 0: Release the USB bus from K state."]
    #[inline(always)]
    pub fn rwakeup(&self) -> RWAKEUP_R {
        RWAKEUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable USB controller. 0: Disable USB controller."]
    #[inline(always)]
    pub fn usb_en(&self) -> USB_EN_R {
        USB_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull-up resistor on USB_DP enable bit 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn dppu_en(&self) -> DPPU_EN_R {
        DPPU_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1: Turn-on related circuit of PHY transceiver 0: power-down related circuit of PHY transceiver"]
    #[inline(always)]
    pub fn pwrdn(&self) -> PWRDN_R {
        PWRDN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1: Byte Mode. The size of the transfer from CPU to USB SRAM can be Byte only. 0: Word Mode. The size of the transfer from CPU to USB SRAM can be Word. only"]
    #[inline(always)]
    pub fn bytem(&self) -> BYTEM_R {
        BYTEM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 1: Enable PHY transceiver function. 0: Disable PHY transceiver function."]
    #[inline(always)]
    pub fn phy_en(&mut self) -> PHY_EN_W {
        PHY_EN_W { w: self }
    }
    #[doc = "Bit 5 - 1: Force USB bus to K state, used for remote wake-up. 0: Release the USB bus from K state."]
    #[inline(always)]
    pub fn rwakeup(&mut self) -> RWAKEUP_W {
        RWAKEUP_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable USB controller. 0: Disable USB controller."]
    #[inline(always)]
    pub fn usb_en(&mut self) -> USB_EN_W {
        USB_EN_W { w: self }
    }
    #[doc = "Bit 8 - Pull-up resistor on USB_DP enable bit 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn dppu_en(&mut self) -> DPPU_EN_W {
        DPPU_EN_W { w: self }
    }
    #[doc = "Bit 9 - 1: Turn-on related circuit of PHY transceiver 0: power-down related circuit of PHY transceiver"]
    #[inline(always)]
    pub fn pwrdn(&mut self) -> PWRDN_W {
        PWRDN_W { w: self }
    }
    #[doc = "Bit 10 - 1: Byte Mode. The size of the transfer from CPU to USB SRAM can be Byte only. 0: Word Mode. The size of the transfer from CPU to USB SRAM can be Word. only"]
    #[inline(always)]
    pub fn bytem(&mut self) -> BYTEM_W {
        BYTEM_W { w: self }
    }
}
