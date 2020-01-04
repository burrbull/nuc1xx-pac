#[doc = "Reader of register CAN_TEST"]
pub type R = crate::R<u32, super::CAN_TEST>;
#[doc = "Writer for register CAN_TEST"]
pub type W = crate::W<u32, super::CAN_TEST>;
#[doc = "Register CAN_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Res`"]
pub type RES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Res`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `Basic`"]
pub type BASIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Basic`"]
pub struct BASIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_W<'a> {
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
#[doc = "Reader of field `Silent`"]
pub type SILENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Silent`"]
pub struct SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SILENT_W<'a> {
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
#[doc = "Reader of field `LBack`"]
pub type LBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBack`"]
pub struct LBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBACK_W<'a> {
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
#[doc = "Reader of field `Tx`"]
pub type TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Tx`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `Rx`"]
pub type RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Rx`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Reserved There are reserved bits. These bits are always read as '0' and must always be written with '0'."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Basic Mode 1= IF1 Registers used as Tx Buffer, IF2 Registers used as Rx Buffer. 0 = Basic Mode disabled."]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Silent Mode 1 = The module is in Silent Mode. 0 = Normal operation."]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode 1 = Loop Back Mode is enabled. 0 = Loop Back Mode is disabled."]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Tx\\[1:0\\]: Control of CAN_TX pin 00 = Reset value, CAN_TX is controlled by the CAN Core 01 = Sample Point can be monitored at CAN_TX pin 10 = CAN_TX pin drives a dominant ('0') value. 11 = CAN_TX pin drives a recessive ('1') value."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Monitors the actual value of CAN_RX Pin (Read Only) 1 = The CAN bus is recessive (CAN_RX = '1'). 0 = The CAN bus is dominant (CAN_RX = '0')."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved There are reserved bits. These bits are always read as '0' and must always be written with '0'."]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bit 2 - Basic Mode 1= IF1 Registers used as Tx Buffer, IF2 Registers used as Rx Buffer. 0 = Basic Mode disabled."]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W { w: self }
    }
    #[doc = "Bit 3 - Silent Mode 1 = The module is in Silent Mode. 0 = Normal operation."]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W { w: self }
    }
    #[doc = "Bit 4 - Loop Back Mode 1 = Loop Back Mode is enabled. 0 = Loop Back Mode is disabled."]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W { w: self }
    }
    #[doc = "Bits 5:6 - Tx\\[1:0\\]: Control of CAN_TX pin 00 = Reset value, CAN_TX is controlled by the CAN Core 01 = Sample Point can be monitored at CAN_TX pin 10 = CAN_TX pin drives a dominant ('0') value. 11 = CAN_TX pin drives a recessive ('1') value."]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 7 - Monitors the actual value of CAN_RX Pin (Read Only) 1 = The CAN bus is recessive (CAN_RX = '1'). 0 = The CAN bus is dominant (CAN_RX = '0')."]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
