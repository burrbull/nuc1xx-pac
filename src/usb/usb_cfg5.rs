#[doc = "Reader of register USB_CFG5"]
pub type R = crate::R<u32, super::USB_CFG5>;
#[doc = "Writer for register USB_CFG5"]
pub type W = crate::W<u32, super::USB_CFG5>;
#[doc = "Register USB_CFG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_CFG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_NUM`"]
pub type EP_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP_NUM`"]
pub struct EP_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ISOCH`"]
pub type ISOCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOCH`"]
pub struct ISOCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCH_W<'a> {
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
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `DSQ_SYNC`"]
pub type DSQ_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSQ_SYNC`"]
pub struct DSQ_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSQ_SYNC_W<'a> {
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
#[doc = "Reader of field `CSTALL`"]
pub type CSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSTALL`"]
pub struct CSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTALL_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - These bits are used to define the endpoint number of the current endpoint."]
    #[inline(always)]
    pub fn ep_num(&self) -> EP_NUM_R {
        EP_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This bit is used to set the endpoint as Isochronous endpoint, no handshake. 1: Isochronous endpoint 0: No Isochronous endpoint"]
    #[inline(always)]
    pub fn isoch(&self) -> ISOCH_R {
        ISOCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 00 = Endpoint is disabled 01 = OUT endpoint 10 = IN endpoint 11 = Undefined"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - 1 = DATA1 PID 0 = DATA0 PID It is used to specify the DATA0 or DATA1 PID in the following IN token transaction. H/W will toggle automatically in IN token base on the bit."]
    #[inline(always)]
    pub fn dsq_sync(&self) -> DSQ_SYNC_R {
        DSQ_SYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 = Clear the device to response STALL handshake in setup stage 0 = Disable the device to clear the STALL handshake in setup stage"]
    #[inline(always)]
    pub fn cstall(&self) -> CSTALL_R {
        CSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits are used to define the endpoint number of the current endpoint."]
    #[inline(always)]
    pub fn ep_num(&mut self) -> EP_NUM_W {
        EP_NUM_W { w: self }
    }
    #[doc = "Bit 4 - This bit is used to set the endpoint as Isochronous endpoint, no handshake. 1: Isochronous endpoint 0: No Isochronous endpoint"]
    #[inline(always)]
    pub fn isoch(&mut self) -> ISOCH_W {
        ISOCH_W { w: self }
    }
    #[doc = "Bits 5:6 - 00 = Endpoint is disabled 01 = OUT endpoint 10 = IN endpoint 11 = Undefined"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 7 - 1 = DATA1 PID 0 = DATA0 PID It is used to specify the DATA0 or DATA1 PID in the following IN token transaction. H/W will toggle automatically in IN token base on the bit."]
    #[inline(always)]
    pub fn dsq_sync(&mut self) -> DSQ_SYNC_W {
        DSQ_SYNC_W { w: self }
    }
    #[doc = "Bit 9 - 1 = Clear the device to response STALL handshake in setup stage 0 = Disable the device to clear the STALL handshake in setup stage"]
    #[inline(always)]
    pub fn cstall(&mut self) -> CSTALL_W {
        CSTALL_W { w: self }
    }
}
