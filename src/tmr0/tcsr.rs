#[doc = "Reader of register TCSR"]
pub type R = crate::R<u32, super::TCSR>;
#[doc = "Writer for register TCSR"]
pub type W = crate::W<u32, super::TCSR>;
#[doc = "Register TCSR `reset()`'s with value 0x05"]
impl crate::ResetValue for super::TCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TDR_EN`"]
pub type TDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDR_EN`"]
pub struct TDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTB`"]
pub type CTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTB`"]
pub struct CTB_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CACT`"]
pub type CACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRST`"]
pub type CRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRST`"]
pub struct CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Pre-scale Counter Clock input is divided by PRESCALE+1 before it is fed to the counter. If PRESCALE=0, then there is no scaling."]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Data Load Enable When TDR_EN is set, TDR (Timer Data Register) will be updated continuously with the 24-bit up-timer value as the timer is counting. 1 = Timer Data Register update enable. 0 = Timer Data Register update disable."]
    #[inline(always)]
    pub fn tdr_en(&self) -> TDR_EN_R {
        TDR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Counter Mode Enable Bit (Low Density only) This bit is the counter mode enable bit. When Timer is used as an event counter, this bit should be set to 1 and Timer will work as an event counter triggered by raising edge of external pin. 1 = Enable counter mode 0 = Disable counter mode"]
    #[inline(always)]
    pub fn ctb(&self) -> CTB_R {
        CTB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer Active Status Bit (Read only) This bit indicates the up-timer status. 0 = Timer is not active. 1 = Timer is active."]
    #[inline(always)]
    pub fn cact(&self) -> CACT_R {
        CACT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer Reset Bit Set this bit will reset the 24-bit up-timer, 8-bit pre-scale counter and also force CEN to 0. 0 = No effect. 1 = Reset Timer's 8-bit pre-scale counter, internal 24-bit up-timer and CEN bit."]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Timer Operating Mode MODE Timer Operating Mode 00 The timer is operating in the one-shot mode. The associated interrupt signal is generated once (if IE is enabled) and CEN is automatically cleared by hardware. 01 The timer is operating in the periodic mode. The associated interrupt signal is generated periodically (if IE is enabled). 10 The timer is operating in the toggle mode. The interrupt signal is generated periodically (if IE is enabled). And the associated signal (tout) is changing back and forth with 50% duty cycle. (This mode only supported in Low Density) 11 The timer is operating in auto-reload counting mode. The associated interrupt signal is generated when TDR = TCMPR (if IE is enabled); however, the 24-bit up-timer counts continuously without reset. (This mode only supported in Low Density)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Interrupt Enable Bit 1 = Enable timer Interrupt. 0 = Disable timer Interrupt. If timer interrupt is enabled, the timer asserts its interrupt signal when the associated up-timer value is equal to TCMPR."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Timer Enable Bit 1 = Starts counting 0 = Stops/Suspends counting Note1: In stop status, and then set CEN to 1 will enables the 24-bit up-timer keeps up counting from the last stop counting value. Note2: This bit is auto-cleared by hardware in one-shot mode (MODE\\[28:27\\]=00) when the associated timer interrupt is generated (IE\\[29\\]=1)."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pre-scale Counter Clock input is divided by PRESCALE+1 before it is fed to the counter. If PRESCALE=0, then there is no scaling."]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 16 - Data Load Enable When TDR_EN is set, TDR (Timer Data Register) will be updated continuously with the 24-bit up-timer value as the timer is counting. 1 = Timer Data Register update enable. 0 = Timer Data Register update disable."]
    #[inline(always)]
    pub fn tdr_en(&mut self) -> TDR_EN_W {
        TDR_EN_W { w: self }
    }
    #[doc = "Bit 24 - Counter Mode Enable Bit (Low Density only) This bit is the counter mode enable bit. When Timer is used as an event counter, this bit should be set to 1 and Timer will work as an event counter triggered by raising edge of external pin. 1 = Enable counter mode 0 = Disable counter mode"]
    #[inline(always)]
    pub fn ctb(&mut self) -> CTB_W {
        CTB_W { w: self }
    }
    #[doc = "Bit 26 - Timer Reset Bit Set this bit will reset the 24-bit up-timer, 8-bit pre-scale counter and also force CEN to 0. 0 = No effect. 1 = Reset Timer's 8-bit pre-scale counter, internal 24-bit up-timer and CEN bit."]
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W {
        CRST_W { w: self }
    }
    #[doc = "Bits 27:28 - Timer Operating Mode MODE Timer Operating Mode 00 The timer is operating in the one-shot mode. The associated interrupt signal is generated once (if IE is enabled) and CEN is automatically cleared by hardware. 01 The timer is operating in the periodic mode. The associated interrupt signal is generated periodically (if IE is enabled). 10 The timer is operating in the toggle mode. The interrupt signal is generated periodically (if IE is enabled). And the associated signal (tout) is changing back and forth with 50% duty cycle. (This mode only supported in Low Density) 11 The timer is operating in auto-reload counting mode. The associated interrupt signal is generated when TDR = TCMPR (if IE is enabled); however, the 24-bit up-timer counts continuously without reset. (This mode only supported in Low Density)"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt Enable Bit 1 = Enable timer Interrupt. 0 = Disable timer Interrupt. If timer interrupt is enabled, the timer asserts its interrupt signal when the associated up-timer value is equal to TCMPR."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 30 - Timer Enable Bit 1 = Starts counting 0 = Stops/Suspends counting Note1: In stop status, and then set CEN to 1 will enables the 24-bit up-timer keeps up counting from the last stop counting value. Note2: This bit is auto-cleared by hardware in one-shot mode (MODE\\[28:27\\]=00) when the associated timer interrupt is generated (IE\\[29\\]=1)."]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
}
