#[doc = "Reader of register PWRCON"]
pub type R = crate::R<u32, super::PWRCON>;
#[doc = "Writer for register PWRCON"]
pub type W = crate::W<u32, super::PWRCON>;
#[doc = "Register PWRCON `reset()`'s with value 0x10"]
impl crate::ResetValue for super::PWRCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `XTL12M_EN`"]
pub type XTL12M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL12M_EN`"]
pub struct XTL12M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL12M_EN_W<'a> {
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
#[doc = "Reader of field `XTL32K_EN`"]
pub type XTL32K_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTL32K_EN`"]
pub struct XTL32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL32K_EN_W<'a> {
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
#[doc = "Reader of field `OSC22M_EN`"]
pub type OSC22M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC22M_EN`"]
pub struct OSC22M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC22M_EN_W<'a> {
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
#[doc = "Reader of field `OSC10K_EN`"]
pub type OSC10K_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC10K_EN`"]
pub struct OSC10K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC10K_EN_W<'a> {
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
#[doc = "Reader of field `PD_WU_DLY`"]
pub type PD_WU_DLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_WU_DLY`"]
pub struct PD_WU_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_WU_DLY_W<'a> {
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
#[doc = "Reader of field `PD_WU_INT_EN`"]
pub type PD_WU_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_WU_INT_EN`"]
pub struct PD_WU_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_WU_INT_EN_W<'a> {
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
#[doc = "Reader of field `PD_WU_STS`"]
pub type PD_WU_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_WU_STS`"]
pub struct PD_WU_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_WU_STS_W<'a> {
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
#[doc = "Reader of field `PWR_DOWN_EN`"]
pub type PWR_DOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWR_DOWN_EN`"]
pub struct PWR_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DOWN_EN_W<'a> {
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
#[doc = "Reader of field `PD_WAIT_CPU`"]
pub type PD_WAIT_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_WAIT_CPU`"]
pub struct PD_WAIT_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_WAIT_CPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External 4~24 MHz Crystal Enable (write-protection bit) The bit default value is set by flash controller user configuration register config0 \\[26:24\\]. When the default clock source is from external 4~24 MHz crystal, this bit is set to 1 automatically 1 = Enable external 4~24 MHz crystal 0 = Disable external 4~24 MHz crystal"]
    #[inline(always)]
    pub fn xtl12m_en(&self) -> XTL12M_EN_R {
        XTL12M_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External 32.768 KHz Crystal Enable (write-protection bit) 1 = Enable external 32.768 kHz Crystal (Normal operation) 0 = Disable external 32.768 kHz Crystal"]
    #[inline(always)]
    pub fn xtl32k_en(&self) -> XTL32K_EN_R {
        XTL32K_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal 22.1184MHz Oscillator Enable (write-protection bit) 1 = Enable 22.1184MHz Oscillation 0 = Disable 22.1184MHz Oscillation"]
    #[inline(always)]
    pub fn osc22m_en(&self) -> OSC22M_EN_R {
        OSC22M_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Internal 10KHz Oscillator Enable (write-protection bit) 1 = Enable 10KHz Oscillation 0 = Disable 10KHz Oscillation"]
    #[inline(always)]
    pub fn osc10k_en(&self) -> OSC10K_EN_R {
        OSC10K_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable the wake up delay counter (write-protection bit) When the chip wakes up from power down mode, the clock control will delay certain clock cycles to wait system clock stable. The delayed clock cycle is 4096 clock cycles when chip work at external 4~24 MHz crystal, and 256 clock cycles when chip work at internal 22.1184 MHz oscillator. 1 = Enable clock cycles delay 0 = Disable clock cycles delay"]
    #[inline(always)]
    pub fn pd_wu_dly(&self) -> PD_WU_DLY_R {
        PD_WU_DLY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power down mode wake up Interrupt enable (write-protection bit) 0 = Disable 1 = Enable. The interrupt will occur when both PD_WU_STS and PD_WU_INT_EN are high."]
    #[inline(always)]
    pub fn pd_wu_int_en(&self) -> PD_WU_INT_EN_R {
        PD_WU_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power down mode wake up interrupt status Set by \"power down wake up\", it indicates that resume from power down mode The flag is set if the GPIO, USB, UART, WDT, CAN, ACMP, BOD or RTC wakeup occurred Write 1 to clear the bit Note: This bit is working only if PD_WU_INT_EN (PWRCON\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pd_wu_sts(&self) -> PD_WU_STS_R {
        PD_WU_STS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System power down enable bit (write-protection bit) When CPU sets this bit \"1\" the chip power down mode is enabled, and chip power-down behavior will depends on the PD_WAIT_CPU bit. (a) If the PD_WAIT_CPU is \"0\", then the chip enters power down mode immediately after the PWR_DOWN_EN bit set. (b) if the PD_WAIT_CPU is \"1\", then the chip keeps active till the CPU sleep mode is also active and then the chip enters power down mode. When chip wakes up from power down mode, this bit is auto cleared. Users need to set this bit again for next power down. When in power down mode, external 4~24 MHz crystal and the internal 22.1184 MHz oscillator will be disabled in this mode, but the external 32 kHz crystal and internal 10 kHz oscillator are not controlled by power down mode. When in power down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by power down mode, if the peripheral clock source is from 32 kHz crystal or the 10 kHz oscillator. 1 = Chip enter the power down mode instant or wait CPU sleep command WFI. 0 = Chip operate in normal mode or CPU in idle mode (sleep mode) because of WFI command."]
    #[inline(always)]
    pub fn pwr_down_en(&self) -> PWR_DOWN_EN_R {
        PWR_DOWN_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit control the power down entry condition (write-protection bit) 1 = Chip enter power down mode when the both PWR_DOWN_EN bit is set to 1 and CPU run WFI instruction. 0 = Chip entry power down mode when the PWR_DOWN_EN bit is set to 1."]
    #[inline(always)]
    pub fn pd_wait_cpu(&self) -> PD_WAIT_CPU_R {
        PD_WAIT_CPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External 4~24 MHz Crystal Enable (write-protection bit) The bit default value is set by flash controller user configuration register config0 \\[26:24\\]. When the default clock source is from external 4~24 MHz crystal, this bit is set to 1 automatically 1 = Enable external 4~24 MHz crystal 0 = Disable external 4~24 MHz crystal"]
    #[inline(always)]
    pub fn xtl12m_en(&mut self) -> XTL12M_EN_W {
        XTL12M_EN_W { w: self }
    }
    #[doc = "Bit 1 - External 32.768 KHz Crystal Enable (write-protection bit) 1 = Enable external 32.768 kHz Crystal (Normal operation) 0 = Disable external 32.768 kHz Crystal"]
    #[inline(always)]
    pub fn xtl32k_en(&mut self) -> XTL32K_EN_W {
        XTL32K_EN_W { w: self }
    }
    #[doc = "Bit 2 - Internal 22.1184MHz Oscillator Enable (write-protection bit) 1 = Enable 22.1184MHz Oscillation 0 = Disable 22.1184MHz Oscillation"]
    #[inline(always)]
    pub fn osc22m_en(&mut self) -> OSC22M_EN_W {
        OSC22M_EN_W { w: self }
    }
    #[doc = "Bit 3 - Internal 10KHz Oscillator Enable (write-protection bit) 1 = Enable 10KHz Oscillation 0 = Disable 10KHz Oscillation"]
    #[inline(always)]
    pub fn osc10k_en(&mut self) -> OSC10K_EN_W {
        OSC10K_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the wake up delay counter (write-protection bit) When the chip wakes up from power down mode, the clock control will delay certain clock cycles to wait system clock stable. The delayed clock cycle is 4096 clock cycles when chip work at external 4~24 MHz crystal, and 256 clock cycles when chip work at internal 22.1184 MHz oscillator. 1 = Enable clock cycles delay 0 = Disable clock cycles delay"]
    #[inline(always)]
    pub fn pd_wu_dly(&mut self) -> PD_WU_DLY_W {
        PD_WU_DLY_W { w: self }
    }
    #[doc = "Bit 5 - Power down mode wake up Interrupt enable (write-protection bit) 0 = Disable 1 = Enable. The interrupt will occur when both PD_WU_STS and PD_WU_INT_EN are high."]
    #[inline(always)]
    pub fn pd_wu_int_en(&mut self) -> PD_WU_INT_EN_W {
        PD_WU_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Power down mode wake up interrupt status Set by \"power down wake up\", it indicates that resume from power down mode The flag is set if the GPIO, USB, UART, WDT, CAN, ACMP, BOD or RTC wakeup occurred Write 1 to clear the bit Note: This bit is working only if PD_WU_INT_EN (PWRCON\\[5\\]) set to 1."]
    #[inline(always)]
    pub fn pd_wu_sts(&mut self) -> PD_WU_STS_W {
        PD_WU_STS_W { w: self }
    }
    #[doc = "Bit 7 - System power down enable bit (write-protection bit) When CPU sets this bit \"1\" the chip power down mode is enabled, and chip power-down behavior will depends on the PD_WAIT_CPU bit. (a) If the PD_WAIT_CPU is \"0\", then the chip enters power down mode immediately after the PWR_DOWN_EN bit set. (b) if the PD_WAIT_CPU is \"1\", then the chip keeps active till the CPU sleep mode is also active and then the chip enters power down mode. When chip wakes up from power down mode, this bit is auto cleared. Users need to set this bit again for next power down. When in power down mode, external 4~24 MHz crystal and the internal 22.1184 MHz oscillator will be disabled in this mode, but the external 32 kHz crystal and internal 10 kHz oscillator are not controlled by power down mode. When in power down mode, the PLL and system clock are disabled, and ignored the clock source selection. The clocks of peripheral are not controlled by power down mode, if the peripheral clock source is from 32 kHz crystal or the 10 kHz oscillator. 1 = Chip enter the power down mode instant or wait CPU sleep command WFI. 0 = Chip operate in normal mode or CPU in idle mode (sleep mode) because of WFI command."]
    #[inline(always)]
    pub fn pwr_down_en(&mut self) -> PWR_DOWN_EN_W {
        PWR_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 8 - This bit control the power down entry condition (write-protection bit) 1 = Chip enter power down mode when the both PWR_DOWN_EN bit is set to 1 and CPU run WFI instruction. 0 = Chip entry power down mode when the PWR_DOWN_EN bit is set to 1."]
    #[inline(always)]
    pub fn pd_wait_cpu(&mut self) -> PD_WAIT_CPU_W {
        PD_WAIT_CPU_W { w: self }
    }
}
