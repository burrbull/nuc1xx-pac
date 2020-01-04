#[doc = "Reader of register USB_DRVSE0"]
pub type R = crate::R<u32, super::USB_DRVSE0>;
#[doc = "Writer for register USB_DRVSE0"]
pub type W = crate::W<u32, super::USB_DRVSE0>;
#[doc = "Register USB_DRVSE0 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::USB_DRVSE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DRVSE0`"]
pub type DRVSE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRVSE0`"]
pub struct DRVSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSE0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The Single Ended Zero (SE0) is when both lines (USB_DP and USB_DM) are being pulled low. 1 = Force USB PHY transceiver to drive SE0 0 = None"]
    #[inline(always)]
    pub fn drvse0(&self) -> DRVSE0_R {
        DRVSE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The Single Ended Zero (SE0) is when both lines (USB_DP and USB_DM) are being pulled low. 1 = Force USB PHY transceiver to drive SE0 0 = None"]
    #[inline(always)]
    pub fn drvse0(&mut self) -> DRVSE0_W {
        DRVSE0_W { w: self }
    }
}
