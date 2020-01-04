#[doc = "Reader of register UA_IRCR"]
pub type R = crate::R<u32, super::UA_IRCR>;
#[doc = "Writer for register UA_IRCR"]
pub type W = crate::W<u32, super::UA_IRCR>;
#[doc = "Register UA_IRCR `reset()`'s with value 0x40"]
impl crate::ResetValue for super::UA_IRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `TX_SELECT`"]
pub type TX_SELECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_SELECT`"]
pub struct TX_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SELECT_W<'a> {
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
#[doc = "Reader of field `INV_TX`"]
pub type INV_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV_TX`"]
pub struct INV_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_TX_W<'a> {
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
#[doc = "Reader of field `INV_RX`"]
pub type INV_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV_RX`"]
pub struct INV_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_RX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable IrDA Receiver 1: Enable IrDA transmitter 0: Enable IrDA receiver"]
    #[inline(always)]
    pub fn tx_select(&self) -> TX_SELECT_R {
        TX_SELECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INV_TX 1= Inverse TX output signal 0= No inversion"]
    #[inline(always)]
    pub fn inv_tx(&self) -> INV_TX_R {
        INV_TX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INV_RX 1= Inverse RX input signal 0= No inversion"]
    #[inline(always)]
    pub fn inv_rx(&self) -> INV_RX_R {
        INV_RX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable IrDA Receiver 1: Enable IrDA transmitter 0: Enable IrDA receiver"]
    #[inline(always)]
    pub fn tx_select(&mut self) -> TX_SELECT_W {
        TX_SELECT_W { w: self }
    }
    #[doc = "Bit 5 - INV_TX 1= Inverse TX output signal 0= No inversion"]
    #[inline(always)]
    pub fn inv_tx(&mut self) -> INV_TX_W {
        INV_TX_W { w: self }
    }
    #[doc = "Bit 6 - INV_RX 1= Inverse RX input signal 0= No inversion"]
    #[inline(always)]
    pub fn inv_rx(&mut self) -> INV_RX_W {
        INV_RX_W { w: self }
    }
}
