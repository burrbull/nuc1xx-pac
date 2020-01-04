#[doc = "Reader of register CAN_STATUS"]
pub type R = crate::R<u32, super::CAN_STATUS>;
#[doc = "Writer for register CAN_STATUS"]
pub type W = crate::W<u32, super::CAN_STATUS>;
#[doc = "Register CAN_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TxOK`"]
pub type TXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxOK`"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
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
#[doc = "Reader of field `RxOK`"]
pub type RXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxOK`"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
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
#[doc = "Reader of field `EPass`"]
pub type EPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPass`"]
pub struct EPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPASS_W<'a> {
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
#[doc = "Reader of field `EWarn`"]
pub type EWARN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWarn`"]
pub struct EWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> EWARN_W<'a> {
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
#[doc = "Reader of field `BOff`"]
pub type BOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOff`"]
pub struct BOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF_W<'a> {
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
    #[doc = "Bits 0:2 - Last Error Code (Type of the last error to occur on the CAN bus) The LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. Table 5-17 describes the error codes."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully 1 = Since this bit was last reset by the CPU, a message has been successfully (error free and acknowledged by at least one other node) transmitted. 0 = Since this bit was reset by the CPU, no message has been successfully transmitted. This bit is never reset by the CAN Core."]
    #[inline(always)]
    pub fn tx_ok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully 1 = A message has been successfully received since this bit was last reset by the CPU (independent of the result of acceptance filtering). 0 = No message has been successfully received since this bit was last reset by the CPU. This bit is never reset by the CAN Core."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Passive (Read Only) 1 = The CAN Core is in the error passive state as defined in the CAN Specification. 0 = The CAN Core is error active."]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status (Read Only) 1 = At least one of the error counters in the EML has reached the error warning limit of 96. 0 = Both error counters are below the error warning limit of 96."]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busoff Status (Read Only) 1 = The CAN module is in busoff state. 0 = The CAN module is not in busoff state."]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code (Type of the last error to occur on the CAN bus) The LEC field holds a code, which indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. The unused code '7' may be written by the CPU to check for updates. Table 5-17 describes the error codes."]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully 1 = Since this bit was last reset by the CPU, a message has been successfully (error free and acknowledged by at least one other node) transmitted. 0 = Since this bit was reset by the CPU, no message has been successfully transmitted. This bit is never reset by the CAN Core."]
    #[inline(always)]
    pub fn tx_ok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Received a Message Successfully 1 = A message has been successfully received since this bit was last reset by the CPU (independent of the result of acceptance filtering). 0 = No message has been successfully received since this bit was last reset by the CPU. This bit is never reset by the CAN Core."]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Bit 5 - Error Passive (Read Only) 1 = The CAN Core is in the error passive state as defined in the CAN Specification. 0 = The CAN Core is error active."]
    #[inline(always)]
    pub fn epass(&mut self) -> EPASS_W {
        EPASS_W { w: self }
    }
    #[doc = "Bit 6 - Error Warning Status (Read Only) 1 = At least one of the error counters in the EML has reached the error warning limit of 96. 0 = Both error counters are below the error warning limit of 96."]
    #[inline(always)]
    pub fn ewarn(&mut self) -> EWARN_W {
        EWARN_W { w: self }
    }
    #[doc = "Bit 7 - Busoff Status (Read Only) 1 = The CAN module is in busoff state. 0 = The CAN module is not in busoff state."]
    #[inline(always)]
    pub fn boff(&mut self) -> BOFF_W {
        BOFF_W { w: self }
    }
}
