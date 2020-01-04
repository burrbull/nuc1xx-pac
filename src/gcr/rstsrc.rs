#[doc = "Reader of register RSTSRC"]
pub type R = crate::R<u32, super::RSTSRC>;
#[doc = "Writer for register RSTSRC"]
pub type W = crate::W<u32, super::RSTSRC>;
#[doc = "Register RSTSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTS_POR`"]
pub type RSTS_POR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_POR`"]
pub struct RSTS_POR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_POR_W<'a> {
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
#[doc = "Reader of field `RSTS_RESET`"]
pub type RSTS_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_RESET`"]
pub struct RSTS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_RESET_W<'a> {
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
#[doc = "Reader of field `RSTS_WDT`"]
pub type RSTS_WDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_WDT`"]
pub struct RSTS_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_WDT_W<'a> {
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
#[doc = "Reader of field `RSTS_LVR`"]
pub type RSTS_LVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_LVR`"]
pub struct RSTS_LVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_LVR_W<'a> {
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
#[doc = "Reader of field `RSTS_BOD`"]
pub type RSTS_BOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_BOD`"]
pub struct RSTS_BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_BOD_W<'a> {
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
#[doc = "Reader of field `RSTS_SYS`"]
pub type RSTS_SYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_SYS`"]
pub struct RSTS_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_SYS_W<'a> {
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
#[doc = "Reader of field `RSTS_CPU`"]
pub type RSTS_CPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS_CPU`"]
pub struct RSTS_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_CPU_W<'a> {
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
    #[doc = "Bit 0 - The RSTS_POR flag is set by the \"reset signal\" from the Power-On Reset (POR) module or bit CHIP_RST (IPRSTC1\\[0\\]) to indicate the previous reset source 1= The Power-On Reset (POR) or CHIP_RST had issued the reset signal to reset the system. 0= No reset from POR or CHIP_RS Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_por(&self) -> RSTS_POR_R {
        RSTS_POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The RSTS_RESET flag is set by the \"reset signal\" from the /RESET pin to indicate the previous reset source. 1 = The Pin /RESET had issued the reset signal to reset the system. 0 = No reset from /RESET pin Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_reset(&self) -> RSTS_RESET_R {
        RSTS_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The The RSTS_WDT flag is set by the \"reset signal\" from the watchdog timer to indicate the previous reset source. 1 = The watchdog timer had issued the reset signal to reset the system. 0 = No reset from watchdog timer Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_wdt(&self) -> RSTS_WDT_R {
        RSTS_WDT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The RSTS_LVR flag is set by the \"reset signal\" from the Low-Voltage-Reset controller to indicate the previous reset source. 1 = The LVR controller had issued the reset signal to reset the system. 0 = No reset from LVR Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_lvr(&self) -> RSTS_LVR_R {
        RSTS_LVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The RSTS_BOD flag is set by the \"reset signal\" from the Brown-Out-Detector controller to indicate the previous reset source. 1 = The BOD had issued the reset signal to reset the system. 0 = No reset from BOD Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_bod(&self) -> RSTS_BOD_R {
        RSTS_BOD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The RSTS_SYS flag is set by the \"reset signal\" from the Cortex_M0 kernel to indicate the previous reset source. 1 = The Cortex_M0 had issued the reset signal to reset the system by software writing 1 to bit SYSRESTREQ(AIRCR\\[2\\], Application Interrupt and Reset Control Register, address = 0xE000ED0C) in system control registers of Cortex_M0 kernel. 0 = No reset from Cortex_M0 Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_sys(&self) -> RSTS_SYS_R {
        RSTS_SYS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The RSTS_CPU flag is set by hardware if software writes CPU_RST (IPRSTC1\\[1\\]) 1 to reset Cortex-M0 CPU kernel and Flash memory controller (FMC). 1 = The Cortex-M0 CPU kernel and FMC are reset by software setting CPU_RST to 1 0 = No reset from CPU Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_cpu(&self) -> RSTS_CPU_R {
        RSTS_CPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The RSTS_POR flag is set by the \"reset signal\" from the Power-On Reset (POR) module or bit CHIP_RST (IPRSTC1\\[0\\]) to indicate the previous reset source 1= The Power-On Reset (POR) or CHIP_RST had issued the reset signal to reset the system. 0= No reset from POR or CHIP_RS Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_por(&mut self) -> RSTS_POR_W {
        RSTS_POR_W { w: self }
    }
    #[doc = "Bit 1 - The RSTS_RESET flag is set by the \"reset signal\" from the /RESET pin to indicate the previous reset source. 1 = The Pin /RESET had issued the reset signal to reset the system. 0 = No reset from /RESET pin Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_reset(&mut self) -> RSTS_RESET_W {
        RSTS_RESET_W { w: self }
    }
    #[doc = "Bit 2 - The The RSTS_WDT flag is set by the \"reset signal\" from the watchdog timer to indicate the previous reset source. 1 = The watchdog timer had issued the reset signal to reset the system. 0 = No reset from watchdog timer Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_wdt(&mut self) -> RSTS_WDT_W {
        RSTS_WDT_W { w: self }
    }
    #[doc = "Bit 3 - The RSTS_LVR flag is set by the \"reset signal\" from the Low-Voltage-Reset controller to indicate the previous reset source. 1 = The LVR controller had issued the reset signal to reset the system. 0 = No reset from LVR Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_lvr(&mut self) -> RSTS_LVR_W {
        RSTS_LVR_W { w: self }
    }
    #[doc = "Bit 4 - The RSTS_BOD flag is set by the \"reset signal\" from the Brown-Out-Detector controller to indicate the previous reset source. 1 = The BOD had issued the reset signal to reset the system. 0 = No reset from BOD Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_bod(&mut self) -> RSTS_BOD_W {
        RSTS_BOD_W { w: self }
    }
    #[doc = "Bit 5 - The RSTS_SYS flag is set by the \"reset signal\" from the Cortex_M0 kernel to indicate the previous reset source. 1 = The Cortex_M0 had issued the reset signal to reset the system by software writing 1 to bit SYSRESTREQ(AIRCR\\[2\\], Application Interrupt and Reset Control Register, address = 0xE000ED0C) in system control registers of Cortex_M0 kernel. 0 = No reset from Cortex_M0 Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_sys(&mut self) -> RSTS_SYS_W {
        RSTS_SYS_W { w: self }
    }
    #[doc = "Bit 7 - The RSTS_CPU flag is set by hardware if software writes CPU_RST (IPRSTC1\\[1\\]) 1 to reset Cortex-M0 CPU kernel and Flash memory controller (FMC). 1 = The Cortex-M0 CPU kernel and FMC are reset by software setting CPU_RST to 1 0 = No reset from CPU Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn rsts_cpu(&mut self) -> RSTS_CPU_W {
        RSTS_CPU_W { w: self }
    }
}
