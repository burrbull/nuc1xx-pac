#[doc = "Reader of register BODCR"]
pub type R = crate::R<u32, super::BODCR>;
#[doc = "Writer for register BODCR"]
pub type W = crate::W<u32, super::BODCR>;
#[doc = "Register BODCR `reset()`'s with value 0x80"]
impl crate::ResetValue for super::BODCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `BOD_EN`"]
pub type BOD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD_EN`"]
pub struct BOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_EN_W<'a> {
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
#[doc = "Reader of field `BOD_VL`"]
pub type BOD_VL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOD_VL`"]
pub struct BOD_VL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_VL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `BOD_RSTEN`"]
pub type BOD_RSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD_RSTEN`"]
pub struct BOD_RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_RSTEN_W<'a> {
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
#[doc = "Reader of field `BOD_INTF`"]
pub type BOD_INTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD_INTF`"]
pub struct BOD_INTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_INTF_W<'a> {
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
#[doc = "Reader of field `BOD_LPM`"]
pub type BOD_LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD_LPM`"]
pub struct BOD_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_LPM_W<'a> {
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
#[doc = "Reader of field `BOD_OUT`"]
pub type BOD_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `LVR_EN`"]
pub type LVR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVR_EN`"]
pub struct LVR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVR_EN_W<'a> {
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
    #[doc = "Bit 0 - Brown Out Detector Enable The default value is set by flash controller user configuration register config0 bit\\[23\\]. 1 = Brown Out Detector function is enabled 0 = Brown Out Detector function is disabled This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_en(&self) -> BOD_EN_R {
        BOD_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Brown Out Detector Threshold Voltage Selection The default value is set by flash controller user configuration register config0 bit\\[22:21\\]. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. BOV_VL\\[1\\]
BOV_VL\\[0\\]
Brown out voltage 1 1 4.5V 1 0 3.8V 0 1 2.7V 0 0 2.2V"]
    #[inline(always)]
    pub fn bod_vl(&self) -> BOD_VL_R {
        BOD_VL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Brown Out Reset 1 = Enable the Brown Out \"RESET\" function. While the Brown Out Detector function is enabled (BOD_EN high) and BOD reset function is enabled (BOD_RSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BOD_OUT high). 0 = Enable the Brown Out \"INTERRUPT\" function While the BOD function is enabled (BOD_EN high) and BOD interrupt function is enabled (BOD_RSTEN low), BOD will assert an interrupt if BOD_OUT is high. BOD interrupt will keep till to the BOD_EN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BOD_EN low). The default value is set by flash controller user configuration register config0 bit\\[20\\]. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_rsten(&self) -> BOD_RSTEN_R {
        BOD_RSTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Brown Out Detector Interrupt Flag 1 = When Brown Out Detector detects the VDD is dropped down through the voltage of BOD_VL setting or the VDD is raised up through the voltage of BOD_VL setting, this bit is set to 1 and the brown out interrupt is requested if brown out interrupt is enabled. 0 = Brown Out Detector does not detect any voltage draft at VDD down through or up through the voltage of BOD_VL setting. Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn bod_intf(&self) -> BOD_INTF_R {
        BOD_INTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Detector Low power Mode 1 = Enable the BOD low power mode 0 = BOD operate in normal mode (default) The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_lpm(&self) -> BOD_LPM_R {
        BOD_LPM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Brown Out Detector output status 1 = Brown Out Detector output status is 1. It means the detected voltage is lower than BOD_VL setting. If the BOD_EN is 0, BOD function disabled , this bit always responds 0 0 = Brown Out Detector output status is 0. It means the detected voltage is higher than BOD_VL setting or BOD_EN is 0"]
    #[inline(always)]
    pub fn bod_out(&self) -> BOD_OUT_R {
        BOD_OUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low Voltage Reset Enable The LVR function reset the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled in default. 1 = Enabled Low Voltage Reset function. After enabling the bit, the LVR function will be active with 100uS delay for LVR output stable (default) 0 = Disabled Low Voltage Reset function This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100"]
    #[inline(always)]
    pub fn lvr_en(&self) -> LVR_EN_R {
        LVR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Brown Out Detector Enable The default value is set by flash controller user configuration register config0 bit\\[23\\]. 1 = Brown Out Detector function is enabled 0 = Brown Out Detector function is disabled This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_en(&mut self) -> BOD_EN_W {
        BOD_EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Brown Out Detector Threshold Voltage Selection The default value is set by flash controller user configuration register config0 bit\\[22:21\\]. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. BOV_VL\\[1\\]
BOV_VL\\[0\\]
Brown out voltage 1 1 4.5V 1 0 3.8V 0 1 2.7V 0 0 2.2V"]
    #[inline(always)]
    pub fn bod_vl(&mut self) -> BOD_VL_W {
        BOD_VL_W { w: self }
    }
    #[doc = "Bit 3 - Brown Out Reset 1 = Enable the Brown Out \"RESET\" function. While the Brown Out Detector function is enabled (BOD_EN high) and BOD reset function is enabled (BOD_RSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BOD_OUT high). 0 = Enable the Brown Out \"INTERRUPT\" function While the BOD function is enabled (BOD_EN high) and BOD interrupt function is enabled (BOD_RSTEN low), BOD will assert an interrupt if BOD_OUT is high. BOD interrupt will keep till to the BOD_EN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BOD_EN low). The default value is set by flash controller user configuration register config0 bit\\[20\\]. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_rsten(&mut self) -> BOD_RSTEN_W {
        BOD_RSTEN_W { w: self }
    }
    #[doc = "Bit 4 - Brown Out Detector Interrupt Flag 1 = When Brown Out Detector detects the VDD is dropped down through the voltage of BOD_VL setting or the VDD is raised up through the voltage of BOD_VL setting, this bit is set to 1 and the brown out interrupt is requested if brown out interrupt is enabled. 0 = Brown Out Detector does not detect any voltage draft at VDD down through or up through the voltage of BOD_VL setting. Software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn bod_intf(&mut self) -> BOD_INTF_W {
        BOD_INTF_W { w: self }
    }
    #[doc = "Bit 5 - Brown Out Detector Low power Mode 1 = Enable the BOD low power mode 0 = BOD operate in normal mode (default) The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response. This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn bod_lpm(&mut self) -> BOD_LPM_W {
        BOD_LPM_W { w: self }
    }
    #[doc = "Bit 7 - Low Voltage Reset Enable The LVR function reset the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled in default. 1 = Enabled Low Voltage Reset function. After enabling the bit, the LVR function will be active with 100uS delay for LVR output stable (default) 0 = Disabled Low Voltage Reset function This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100"]
    #[inline(always)]
    pub fn lvr_en(&mut self) -> LVR_EN_W {
        LVR_EN_W { w: self }
    }
}
