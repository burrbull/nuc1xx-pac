#[doc = "Reader of register CCR0"]
pub type R = crate::R<u32, super::CCR0>;
#[doc = "Writer for register CCR0"]
pub type W = crate::W<u32, super::CCR0>;
#[doc = "Register CCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INV0`"]
pub type INV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV0`"]
pub struct INV0_W<'a> {
    w: &'a mut W,
}
impl<'a> INV0_W<'a> {
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
#[doc = "Reader of field `CRL_IE0`"]
pub type CRL_IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRL_IE0`"]
pub struct CRL_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRL_IE0_W<'a> {
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
#[doc = "Reader of field `CFL_IE0`"]
pub type CFL_IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFL_IE0`"]
pub struct CFL_IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_IE0_W<'a> {
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
#[doc = "Reader of field `CAPCH0EN`"]
pub type CAPCH0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPCH0EN`"]
pub struct CAPCH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCH0EN_W<'a> {
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
#[doc = "Reader of field `CAPIF0`"]
pub type CAPIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPIF0`"]
pub struct CAPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIF0_W<'a> {
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
#[doc = "Reader of field `CRLRI0`"]
pub type CRLRI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRLRI0`"]
pub struct CRLRI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLRI0_W<'a> {
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
#[doc = "Reader of field `CFLRI0`"]
pub type CFLRI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFLRI0`"]
pub struct CFLRI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLRI0_W<'a> {
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
#[doc = "Reader of field `INV1`"]
pub type INV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV1`"]
pub struct INV1_W<'a> {
    w: &'a mut W,
}
impl<'a> INV1_W<'a> {
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
#[doc = "Reader of field `CRL_IE1`"]
pub type CRL_IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRL_IE1`"]
pub struct CRL_IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CRL_IE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CFL_IE1`"]
pub type CFL_IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFL_IE1`"]
pub struct CFL_IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_IE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CAPCH1EN`"]
pub type CAPCH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPCH1EN`"]
pub struct CAPCH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCH1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CAPIF1`"]
pub type CAPIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPIF1`"]
pub struct CAPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIF1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CRLRI1`"]
pub type CRLRI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRLRI1`"]
pub struct CRLRI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLRI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CFLRI1`"]
pub type CFLRI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFLRI1`"]
pub struct CFLRI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLRI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv0(&self) -> INV0_R {
        INV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 0 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie0(&self) -> CRL_IE0_R {
        CRL_IE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 0 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie0(&self) -> CFL_IE0_R {
        CFL_IE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Capture Function Enable 1 = Enable capture function on PWM group channel 0. 0 = Disable capture function on PWM group channel 0. When Enable, Capture latched the PWM-counter value and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 0 Interrupt."]
    #[inline(always)]
    pub fn capch0en(&self) -> CAPCH0EN_R {
        CAPCH0EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Capture Interrupt Indication Flag If PWM group channel 0 rising latch interrupt is enabled (CRL_IE0=1), a rising transition occurs at PWM group channel 0 will result in CAPIF0 to high; Similarly, a falling transition will cause CAPIF0 to be set high if PWM group channel 0 falling latch interrupt is enabled (CFL_IE0=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif0(&self) -> CAPIF0_R {
        CAPIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CRLR0 Latched Indicator Bit When PWM group input channel 0 has a rising transition, CRLR0 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri0(&self) -> CRLRI0_R {
        CRLRI0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CFLR0 Latched Indicator Bit When PWM group input channel 0 has a falling transition, CFLR0 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri0(&self) -> CFLRI0_R {
        CFLRI0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 1 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv1(&self) -> INV1_R {
        INV1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 1 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie1(&self) -> CRL_IE1_R {
        CRL_IE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 1 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 1 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie1(&self) -> CFL_IE1_R {
        CFL_IE1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 1 Capture Function Enable 1 = Enable capture function on PWM group channel 1. 0 = Disable capture function on PWM group channel 1. When Enable, Capture latched the PMW-counter and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 1 Interrupt."]
    #[inline(always)]
    pub fn capch1en(&self) -> CAPCH1EN_R {
        CAPCH1EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 1 Capture Interrupt Indication Flag If PWM group channel 1 rising latch interrupt is enabled (CRL_IE1=1), a rising transition occurs at PWM group channel 1 will result in CAPIF1 to high; Similarly, a falling transition will cause CAPIF1 to be set high if PWM group channel 1 falling latch interrupt is enabled (CFL_IE1=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif1(&self) -> CAPIF1_R {
        CAPIF1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CRLR1 Latched Indicator Bit When PWM group input channel 1 has a rising transition, CRLR1 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri1(&self) -> CRLRI1_R {
        CRLRI1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CFLR1 Latched Indicator Bit When PWM group input channel 1 has a falling transition, CFLR1 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri1(&self) -> CFLRI1_R {
        CFLRI1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv0(&mut self) -> INV0_W {
        INV0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 0 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 0 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie0(&mut self) -> CRL_IE0_W {
        CRL_IE0_W { w: self }
    }
    #[doc = "Bit 2 - Channel 0 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 0 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie0(&mut self) -> CFL_IE0_W {
        CFL_IE0_W { w: self }
    }
    #[doc = "Bit 3 - Channel 0 Capture Function Enable 1 = Enable capture function on PWM group channel 0. 0 = Disable capture function on PWM group channel 0. When Enable, Capture latched the PWM-counter value and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 0 Interrupt."]
    #[inline(always)]
    pub fn capch0en(&mut self) -> CAPCH0EN_W {
        CAPCH0EN_W { w: self }
    }
    #[doc = "Bit 4 - Channel 0 Capture Interrupt Indication Flag If PWM group channel 0 rising latch interrupt is enabled (CRL_IE0=1), a rising transition occurs at PWM group channel 0 will result in CAPIF0 to high; Similarly, a falling transition will cause CAPIF0 to be set high if PWM group channel 0 falling latch interrupt is enabled (CFL_IE0=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif0(&mut self) -> CAPIF0_W {
        CAPIF0_W { w: self }
    }
    #[doc = "Bit 6 - CRLR0 Latched Indicator Bit When PWM group input channel 0 has a rising transition, CRLR0 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri0(&mut self) -> CRLRI0_W {
        CRLRI0_W { w: self }
    }
    #[doc = "Bit 7 - CFLR0 Latched Indicator Bit When PWM group input channel 0 has a falling transition, CFLR0 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri0(&mut self) -> CFLRI0_W {
        CFLRI0_W { w: self }
    }
    #[doc = "Bit 16 - Channel 1 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv1(&mut self) -> INV1_W {
        INV1_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 1 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie1(&mut self) -> CRL_IE1_W {
        CRL_IE1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 1 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 1 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie1(&mut self) -> CFL_IE1_W {
        CFL_IE1_W { w: self }
    }
    #[doc = "Bit 19 - Channel 1 Capture Function Enable 1 = Enable capture function on PWM group channel 1. 0 = Disable capture function on PWM group channel 1. When Enable, Capture latched the PMW-counter and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 1 Interrupt."]
    #[inline(always)]
    pub fn capch1en(&mut self) -> CAPCH1EN_W {
        CAPCH1EN_W { w: self }
    }
    #[doc = "Bit 20 - Channel 1 Capture Interrupt Indication Flag If PWM group channel 1 rising latch interrupt is enabled (CRL_IE1=1), a rising transition occurs at PWM group channel 1 will result in CAPIF1 to high; Similarly, a falling transition will cause CAPIF1 to be set high if PWM group channel 1 falling latch interrupt is enabled (CFL_IE1=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif1(&mut self) -> CAPIF1_W {
        CAPIF1_W { w: self }
    }
    #[doc = "Bit 22 - CRLR1 Latched Indicator Bit When PWM group input channel 1 has a rising transition, CRLR1 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri1(&mut self) -> CRLRI1_W {
        CRLRI1_W { w: self }
    }
    #[doc = "Bit 23 - CFLR1 Latched Indicator Bit When PWM group input channel 1 has a falling transition, CFLR1 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri1(&mut self) -> CFLRI1_W {
        CFLRI1_W { w: self }
    }
}
