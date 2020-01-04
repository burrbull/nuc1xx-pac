#[doc = "Reader of register USB_CFGP3"]
pub type R = crate::R<u32, super::USB_CFGP3>;
#[doc = "Writer for register USB_CFGP3"]
pub type W = crate::W<u32, super::USB_CFGP3>;
#[doc = "Register USB_CFGP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_CFGP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLRRDY`"]
pub struct CLRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRRDY_W<'a> {
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
#[doc = "Reader of field `SSTALL`"]
pub type SSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSTALL`"]
pub struct SSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTALL_W<'a> {
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
    #[doc = "Bit 1 - 1 = Set the device to respond STALL automatically 0 = Disable the device to response STALL"]
    #[inline(always)]
    pub fn sstall(&self) -> SSTALL_R {
        SSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the MXPLD register is set by user, it means that the endpoint is ready to transmit or receive data. If the user wants to turn off this transaction before the transaction start, users can set this bit to 1 to turn it off and it is auto clear to 0. For IN token, write 1 is used to clear the IN token had ready to transmit the data to USB. For OUT token, write 1 is used to clear the OUT token had ready to receive the data from USB. This bit is write 1 only and it is always 0 when it was read back."]
    #[inline(always)]
    pub fn clrrdy(&mut self) -> CLRRDY_W {
        CLRRDY_W { w: self }
    }
    #[doc = "Bit 1 - 1 = Set the device to respond STALL automatically 0 = Disable the device to response STALL"]
    #[inline(always)]
    pub fn sstall(&mut self) -> SSTALL_W {
        SSTALL_W { w: self }
    }
}
