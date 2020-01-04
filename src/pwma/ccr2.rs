#[doc = "Reader of register CCR2"]
pub type R = crate::R<u32, super::CCR2>;
#[doc = "Writer for register CCR2"]
pub type W = crate::W<u32, super::CCR2>;
#[doc = "Register CCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INV2`"]
pub type INV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV2`"]
pub struct INV2_W<'a> {
    w: &'a mut W,
}
impl<'a> INV2_W<'a> {
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
#[doc = "Reader of field `CRL_IE2`"]
pub type CRL_IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRL_IE2`"]
pub struct CRL_IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CRL_IE2_W<'a> {
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
#[doc = "Reader of field `CFL_IE2`"]
pub type CFL_IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFL_IE2`"]
pub struct CFL_IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_IE2_W<'a> {
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
#[doc = "Reader of field `CAPCH2EN`"]
pub type CAPCH2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPCH2EN`"]
pub struct CAPCH2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCH2EN_W<'a> {
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
#[doc = "Reader of field `CAPIF2`"]
pub type CAPIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPIF2`"]
pub struct CAPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIF2_W<'a> {
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
#[doc = "Reader of field `CRLRI2`"]
pub type CRLRI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRLRI2`"]
pub struct CRLRI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLRI2_W<'a> {
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
#[doc = "Reader of field `CFLRI2`"]
pub type CFLRI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFLRI2`"]
pub struct CFLRI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLRI2_W<'a> {
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
#[doc = "Reader of field `INV3`"]
pub type INV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV3`"]
pub struct INV3_W<'a> {
    w: &'a mut W,
}
impl<'a> INV3_W<'a> {
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
#[doc = "Reader of field `CRL_IE3`"]
pub type CRL_IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRL_IE3`"]
pub struct CRL_IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> CRL_IE3_W<'a> {
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
#[doc = "Reader of field `CFL_IE3`"]
pub type CFL_IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFL_IE3`"]
pub struct CFL_IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_IE3_W<'a> {
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
#[doc = "Reader of field `CAPCH3EN`"]
pub type CAPCH3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPCH3EN`"]
pub struct CAPCH3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCH3EN_W<'a> {
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
#[doc = "Reader of field `CAPIF3`"]
pub type CAPIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPIF3`"]
pub struct CAPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPIF3_W<'a> {
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
#[doc = "Reader of field `CRLRI3`"]
pub type CRLRI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRLRI3`"]
pub struct CRLRI3_W<'a> {
    w: &'a mut W,
}
impl<'a> CRLRI3_W<'a> {
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
#[doc = "Reader of field `CFLRI3`"]
pub type CFLRI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFLRI3`"]
pub struct CFLRI3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFLRI3_W<'a> {
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
    #[doc = "Bit 0 - Channel 2 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv2(&self) -> INV2_R {
        INV2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 2 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 2 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie2(&self) -> CRL_IE2_R {
        CRL_IE2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 2 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie2(&self) -> CFL_IE2_R {
        CFL_IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Capture Function Enable 1 = Enable capture function on PWM group channel 2. 0 = Disable capture function on PWM group channel 2. When Enable, Capture latched the PWM-counter value and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 2 Interrupt."]
    #[inline(always)]
    pub fn capch2en(&self) -> CAPCH2EN_R {
        CAPCH2EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Capture Interrupt Indication Flag If PWM group channel 2 rising latch interrupt is enabled (CRL_IE2=1), a rising transition occurs at PWM group channel 2 will result in CAPIF2 to high; Similarly, a falling transition will cause CAPIF2 to be set high if PWM group channel 2 falling latch interrupt is enabled (CFL_IE2=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif2(&self) -> CAPIF2_R {
        CAPIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CRLR2 Latched Indicator Bit When PWM group input channel 2 has a rising transition, CRLR2 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri2(&self) -> CRLRI2_R {
        CRLRI2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CFLR2 Latched Indicator Bit When PWM group input channel 2 has a falling transition, CFLR2 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri2(&self) -> CFLRI2_R {
        CFLRI2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 3 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv3(&self) -> INV3_R {
        INV3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 3 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 3 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie3(&self) -> CRL_IE3_R {
        CRL_IE3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 3 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 3 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie3(&self) -> CFL_IE3_R {
        CFL_IE3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Capture Function Enable 1 = Enable capture function on PWM group channel 3. 0 = Disable capture function on PWM group channel 3. When Enable, Capture latched the PMW-counter and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 3 Interrupt."]
    #[inline(always)]
    pub fn capch3en(&self) -> CAPCH3EN_R {
        CAPCH3EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 3 Capture Interrupt Indication Flag If PWM group channel 3 rising latch interrupt is enabled (CRL_IE3=1), a rising transition occurs at PWM group channel 3 will result in CAPIF3 to high; Similarly, a falling transition will cause CAPIF3 to be set high if PWM group channel 3 falling latch interrupt is enabled (CFL_IE3=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif3(&self) -> CAPIF3_R {
        CAPIF3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CRLR3 Latched Indicator Bit When PWM group input channel 3 has a rising transition, CRLR3 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri3(&self) -> CRLRI3_R {
        CRLRI3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CFLR3 Latched Indicator Bit When PWM group input channel 3 has a falling transition, CFLR3 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri3(&self) -> CFLRI3_R {
        CFLRI3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 2 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv2(&mut self) -> INV2_W {
        INV2_W { w: self }
    }
    #[doc = "Bit 1 - Channel 2 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 2 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie2(&mut self) -> CRL_IE2_W {
        CRL_IE2_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 2 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie2(&mut self) -> CFL_IE2_W {
        CFL_IE2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 2 Capture Function Enable 1 = Enable capture function on PWM group channel 2. 0 = Disable capture function on PWM group channel 2. When Enable, Capture latched the PWM-counter value and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 2 Interrupt."]
    #[inline(always)]
    pub fn capch2en(&mut self) -> CAPCH2EN_W {
        CAPCH2EN_W { w: self }
    }
    #[doc = "Bit 4 - Channel 2 Capture Interrupt Indication Flag If PWM group channel 2 rising latch interrupt is enabled (CRL_IE2=1), a rising transition occurs at PWM group channel 2 will result in CAPIF2 to high; Similarly, a falling transition will cause CAPIF2 to be set high if PWM group channel 2 falling latch interrupt is enabled (CFL_IE2=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif2(&mut self) -> CAPIF2_W {
        CAPIF2_W { w: self }
    }
    #[doc = "Bit 6 - CRLR2 Latched Indicator Bit When PWM group input channel 2 has a rising transition, CRLR2 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri2(&mut self) -> CRLRI2_W {
        CRLRI2_W { w: self }
    }
    #[doc = "Bit 7 - CFLR2 Latched Indicator Bit When PWM group input channel 2 has a falling transition, CFLR2 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri2(&mut self) -> CFLRI2_W {
        CFLRI2_W { w: self }
    }
    #[doc = "Bit 16 - Channel 3 Inverter Enable 1 = Inverter enable. Reverse the input signal from GPIO before fed to Capture timer 0 = Inverter disable"]
    #[inline(always)]
    pub fn inv3(&mut self) -> INV3_W {
        INV3_W { w: self }
    }
    #[doc = "Bit 17 - Channel 3 Rising Latch Interrupt Enable 1 = Enable rising latch interrupt 0 = Disable rising latch interrupt When Enable, if Capture detects PWM group channel 3 has rising transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn crl_ie3(&mut self) -> CRL_IE3_W {
        CRL_IE3_W { w: self }
    }
    #[doc = "Bit 18 - Channel 3 Falling Latch Interrupt Enable 1 = Enable falling latch interrupt 0 = Disable falling latch interrupt When Enable, if Capture detects PWM group channel 3 has falling transition, Capture issues an Interrupt."]
    #[inline(always)]
    pub fn cfl_ie3(&mut self) -> CFL_IE3_W {
        CFL_IE3_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Capture Function Enable 1 = Enable capture function on PWM group channel 3. 0 = Disable capture function on PWM group channel 3. When Enable, Capture latched the PMW-counter and saved to CRLR (Rising latch) and CFLR (Falling latch). When Disable, Capture does not update CRLR and CFLR, and disable PWM group channel 3 Interrupt."]
    #[inline(always)]
    pub fn capch3en(&mut self) -> CAPCH3EN_W {
        CAPCH3EN_W { w: self }
    }
    #[doc = "Bit 20 - Channel 3 Capture Interrupt Indication Flag If PWM group channel 3 rising latch interrupt is enabled (CRL_IE3=1), a rising transition occurs at PWM group channel 3 will result in CAPIF3 to high; Similarly, a falling transition will cause CAPIF3 to be set high if PWM group channel 3 falling latch interrupt is enabled (CFL_IE3=1). Write 1 to clear this bit to zero"]
    #[inline(always)]
    pub fn capif3(&mut self) -> CAPIF3_W {
        CAPIF3_W { w: self }
    }
    #[doc = "Bit 22 - CRLR3 Latched Indicator Bit When PWM group input channel 3 has a rising transition, CRLR3 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn crlri3(&mut self) -> CRLRI3_W {
        CRLRI3_W { w: self }
    }
    #[doc = "Bit 23 - CFLR3 Latched Indicator Bit When PWM group input channel 3 has a falling transition, CFLR3 was latched with the value of PWM down-counter and this bit is set by hardware. In Medium Density, software can write 0 to clear this bit to zero. In Low Density, software can write 0 to clear this bit to zero if BCn bit is 0, and can Write 1 to clear this bit to zero if BCn bit is 1."]
    #[inline(always)]
    pub fn cflri3(&mut self) -> CFLRI3_W {
        CFLRI3_W { w: self }
    }
}
