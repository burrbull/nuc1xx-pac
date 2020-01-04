#[doc = "Reader of register UA_LCR"]
pub type R = crate::R<u32, super::UA_LCR>;
#[doc = "Writer for register UA_LCR"]
pub type W = crate::W<u32, super::UA_LCR>;
#[doc = "Register UA_LCR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_LCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NSB`"]
pub type NSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSB`"]
pub struct NSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSB_W<'a> {
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
#[doc = "Reader of field `PBE`"]
pub type PBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBE`"]
pub struct PBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBE_W<'a> {
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
#[doc = "Reader of field `EPE`"]
pub type EPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPE`"]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
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
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
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
#[doc = "Reader of field `BCB`"]
pub type BCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCB`"]
pub struct BCB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCB_W<'a> {
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
    #[doc = "Bits 0:1 - Word Length Select WLS\\[1:0\\]
Character length 00 5 bits 01 6 bits 10 7 bits 11 8 bits"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Number of \"STOP bit\" 0= One \"STOP bit\" is generated in the transmitted data 1= One and a half \"STOP bit\" is generated in the transmitted data when 5-bit word length is selected; Two \"STOP bit\" is generated when 6-, 7- and 8-bit word length is selected."]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Bit Enable 0 = Parity bit is not generated (transmit data) or checked (receive data) during transfer. 1 = Parity bit is generated or checked between the \"last data word bit\" and \"stop bit\" of the serial data."]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Even Parity Enable 0 = Odd number of logic 1's are transmitted or checked in the data word and parity bits. 1 = Even number of logic 1's are transmitted or checked in the data word and parity bits. This bit has effect only when bit 3 (parity bit enable) is set."]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stick Parity Enable 0 = Disable stick parity 1 = When bits PBE , EPE and SPE are set, the parity bit is transmitted and checked as cleared. When PBE and SPE are set and EPE is cleared, the parity bit is transmitted and checked as set."]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select WLS\\[1:0\\]
Character length 00 5 bits 01 6 bits 10 7 bits 11 8 bits"]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - Number of \"STOP bit\" 0= One \"STOP bit\" is generated in the transmitted data 1= One and a half \"STOP bit\" is generated in the transmitted data when 5-bit word length is selected; Two \"STOP bit\" is generated when 6-, 7- and 8-bit word length is selected."]
    #[inline(always)]
    pub fn nsb(&mut self) -> NSB_W {
        NSB_W { w: self }
    }
    #[doc = "Bit 3 - Parity Bit Enable 0 = Parity bit is not generated (transmit data) or checked (receive data) during transfer. 1 = Parity bit is generated or checked between the \"last data word bit\" and \"stop bit\" of the serial data."]
    #[inline(always)]
    pub fn pbe(&mut self) -> PBE_W {
        PBE_W { w: self }
    }
    #[doc = "Bit 4 - Even Parity Enable 0 = Odd number of logic 1's are transmitted or checked in the data word and parity bits. 1 = Even number of logic 1's are transmitted or checked in the data word and parity bits. This bit has effect only when bit 3 (parity bit enable) is set."]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 5 - Stick Parity Enable 0 = Disable stick parity 1 = When bits PBE , EPE and SPE are set, the parity bit is transmitted and checked as cleared. When PBE and SPE are set and EPE is cleared, the parity bit is transmitted and checked as set."]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 6 - Break Control Bit When this bit is set to logic 1, the serial data output (TX) is forced to the Spacing State (logic 0). This bit acts only on TX and has no effect on the transmitter logic."]
    #[inline(always)]
    pub fn bcb(&mut self) -> BCB_W {
        BCB_W { w: self }
    }
}
