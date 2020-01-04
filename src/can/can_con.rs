#[doc = "Reader of register CAN_CON"]
pub type R = crate::R<u32, super::CAN_CON>;
#[doc = "Writer for register CAN_CON"]
pub type W = crate::W<u32, super::CAN_CON>;
#[doc = "Register CAN_CON `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CAN_CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `Init`"]
pub type INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Init`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Reader of field `SIE`"]
pub type SIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIE`"]
pub struct SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIE_W<'a> {
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
#[doc = "Reader of field `EIE`"]
pub type EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIE`"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
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
#[doc = "Reader of field `DAR`"]
pub type DAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAR`"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
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
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
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
#[doc = "Reader of field `Test`"]
pub type TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Test`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
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
    #[doc = "Bit 0 - Init Initialization 1 = Initialization is started. 0 = Normal Operation."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable 1 = Enabled. 0 = Disabled."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable 1 = Enabled - An interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected. 0 = Disabled - No Status Change Interrupt will be generated."]
    #[inline(always)]
    pub fn sie(&self) -> SIE_R {
        SIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable 1 = Enabled - A change in the bits BOff or EWarn in the Status Register will generate an interrupt. 0 = Disabled - No Error Status Interrupt will be generated."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic Re-transmission 1 = Automatic Retransmission disabled. 0 = Automatic Retransmission of disturbed messages enabled."]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable 1 = Write access to the Bit Timing Register (CAN_BTIME & CAN_BRP) allowed. (while Init bit =1). 0 = No write access to the Bit Timing Register."]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable 1 = Test Mode. 0 = Normal Operation."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Init Initialization 1 = Initialization is started. 0 = Normal Operation."]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Module Interrupt Enable 1 = Enabled. 0 = Disabled."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable 1 = Enabled - An interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected. 0 = Disabled - No Status Change Interrupt will be generated."]
    #[inline(always)]
    pub fn sie(&mut self) -> SIE_W {
        SIE_W { w: self }
    }
    #[doc = "Bit 3 - Error Interrupt Enable 1 = Enabled - A change in the bits BOff or EWarn in the Status Register will generate an interrupt. 0 = Disabled - No Error Status Interrupt will be generated."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 5 - Disable Automatic Re-transmission 1 = Automatic Retransmission disabled. 0 = Automatic Retransmission of disturbed messages enabled."]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 6 - Configuration Change Enable 1 = Write access to the Bit Timing Register (CAN_BTIME & CAN_BRP) allowed. (while Init bit =1). 0 = No write access to the Bit Timing Register."]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 7 - Test Mode Enable 1 = Test Mode. 0 = Normal Operation."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
}
