#[doc = "Reader of register ADCR"]
pub type R = crate::R<u32, super::ADCR>;
#[doc = "Writer for register ADCR"]
pub type W = crate::W<u32, super::ADCR>;
#[doc = "Register ADCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADEN`"]
pub type ADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEN`"]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
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
#[doc = "Reader of field `ADIE`"]
pub type ADIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIE`"]
pub struct ADIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIE_W<'a> {
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
#[doc = "Reader of field `ADMD`"]
pub type ADMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADMD`"]
pub struct ADMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TRGS`"]
pub type TRGS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGS`"]
pub struct TRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRGCOND`"]
pub type TRGCOND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGCOND`"]
pub struct TRGCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGCOND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRGEN`"]
pub type TRGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGEN`"]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
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
#[doc = "Reader of field `PTEN`"]
pub type PTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTEN`"]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIFFEN`"]
pub type DIFFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFFEN`"]
pub struct DIFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADST`"]
pub type ADST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADST`"]
pub struct ADST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DMOF`"]
pub type DMOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMOF`"]
pub struct DMOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Enable 1 = Enable 0 = Disable Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit for saving power consumption."]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - A/D Interrupt Enable 1 = Enable A/D interrupt function 0 = Disable A/D interrupt function A/D conversion end interrupt request is generated if ADIE bit is set to 1."]
    #[inline(always)]
    pub fn adie(&self) -> ADIE_R {
        ADIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - A/D Converter Operation Mode 00 = Single conversion 01 = Reserved 10 = Single-cycle scan 11 = Continuous scan When changing the operation mode, software should disable ADST bit firstly."]
    #[inline(always)]
    pub fn admd(&self) -> ADMD_R {
        ADMD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Hardware Trigger Source 00 = A/D conversion is started by external STADC pin. Others = Reserved Software should disable TRGE and ADST before change TRGS. In hardware trigger mode, the ADST bit is set by the external trigger from STADC."]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Trigger Condition These two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and 4 PCLKs at high and low state for edge trigger. 00 = Low level 01 = High level 10 = Falling edge 11 = Positive edge"]
    #[inline(always)]
    pub fn trgcond(&self) -> TRGCOND_R {
        TRGCOND_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Trigger Enable Enable or disable triggering of A/D conversion by external STADC pin. 1= Enable 0= Disable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PDMA Transfer Enable 1 = Enable PDMA data transfer in ADDR 0~7 0 = Disable PDMA data transfer. When A/D conversion is completed, the converted data is loaded into ADDR 0~7, software can enable this bit to generate a PDMA data transfer request. When PTEN=1, software must set ADIE=0 to disable interrupt."]
    #[inline(always)]
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A/D Differential Input Mode Enable 1 = A/D is in differential analog input mode 0 = A/D is in single-end analog input mode Differential input voltage (Vdiff) = Vplus - Vminus The Vplus of differential input paired channel 0 is from ADC0 pin; Vminus is from ADC1 pin. The Vplus of differential input paired channel 1 is from ADC2 pin; Vminus is from ADC3 pin. The Vplus of differential input paired channel 2 is from ADC4 pin; Vminus is from ADC5 pin. The Vplus of differential input paired channel 3 is from ADC6 pin; Vminus is from ADC7 pin. In differential input mode, only one of the two corresponding channels needs to be enabled in ADCHER. The conversion result will be placed to the corresponding data register of the enabled channel. If both channels of a differential input paired channel are enabled, the ADC will convert it twice in scan mode. And then write the conversion result to the two corresponding data registers."]
    #[inline(always)]
    pub fn diffen(&self) -> DIFFEN_R {
        DIFFEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - A/D Conversion Start 1 = Conversion start. 0 = Conversion stopped and A/D converter enter idle state. ADST bit can be controlled by two sources: software write and external pin STADC. ADST is cleared to 0 by hardware automatically at the ends of single mode and single-cycle scan mode on specified channels. In continuous scan mode, A/D conversion is continuously performed sequentially until this bit is cleared to 0 or chip reset."]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 31 - A/D differential input Mode Output Format This bit is only supported in Low Density. 1 = A/D Conversion result will be filled in RSLT at ADDRx registers with 2'complement format. 0 = A/D Conversion result will be filled in RSLT at ADDRx registers with unsigned format."]
    #[inline(always)]
    pub fn dmof(&self) -> DMOF_R {
        DMOF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Enable 1 = Enable 0 = Disable Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit for saving power consumption."]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Bit 1 - A/D Interrupt Enable 1 = Enable A/D interrupt function 0 = Disable A/D interrupt function A/D conversion end interrupt request is generated if ADIE bit is set to 1."]
    #[inline(always)]
    pub fn adie(&mut self) -> ADIE_W {
        ADIE_W { w: self }
    }
    #[doc = "Bits 2:3 - A/D Converter Operation Mode 00 = Single conversion 01 = Reserved 10 = Single-cycle scan 11 = Continuous scan When changing the operation mode, software should disable ADST bit firstly."]
    #[inline(always)]
    pub fn admd(&mut self) -> ADMD_W {
        ADMD_W { w: self }
    }
    #[doc = "Bits 4:5 - Hardware Trigger Source 00 = A/D conversion is started by external STADC pin. Others = Reserved Software should disable TRGE and ADST before change TRGS. In hardware trigger mode, the ADST bit is set by the external trigger from STADC."]
    #[inline(always)]
    pub fn trgs(&mut self) -> TRGS_W {
        TRGS_W { w: self }
    }
    #[doc = "Bits 6:7 - External Trigger Condition These two bits decide external pin STADC trigger event is level or edge. The signal must be kept at stable state at least 8 PCLKs for level trigger and 4 PCLKs at high and low state for edge trigger. 00 = Low level 01 = High level 10 = Falling edge 11 = Positive edge"]
    #[inline(always)]
    pub fn trgcond(&mut self) -> TRGCOND_W {
        TRGCOND_W { w: self }
    }
    #[doc = "Bit 8 - External Trigger Enable Enable or disable triggering of A/D conversion by external STADC pin. 1= Enable 0= Disable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TRGEN_W {
        TRGEN_W { w: self }
    }
    #[doc = "Bit 9 - PDMA Transfer Enable 1 = Enable PDMA data transfer in ADDR 0~7 0 = Disable PDMA data transfer. When A/D conversion is completed, the converted data is loaded into ADDR 0~7, software can enable this bit to generate a PDMA data transfer request. When PTEN=1, software must set ADIE=0 to disable interrupt."]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
    #[doc = "Bit 10 - A/D Differential Input Mode Enable 1 = A/D is in differential analog input mode 0 = A/D is in single-end analog input mode Differential input voltage (Vdiff) = Vplus - Vminus The Vplus of differential input paired channel 0 is from ADC0 pin; Vminus is from ADC1 pin. The Vplus of differential input paired channel 1 is from ADC2 pin; Vminus is from ADC3 pin. The Vplus of differential input paired channel 2 is from ADC4 pin; Vminus is from ADC5 pin. The Vplus of differential input paired channel 3 is from ADC6 pin; Vminus is from ADC7 pin. In differential input mode, only one of the two corresponding channels needs to be enabled in ADCHER. The conversion result will be placed to the corresponding data register of the enabled channel. If both channels of a differential input paired channel are enabled, the ADC will convert it twice in scan mode. And then write the conversion result to the two corresponding data registers."]
    #[inline(always)]
    pub fn diffen(&mut self) -> DIFFEN_W {
        DIFFEN_W { w: self }
    }
    #[doc = "Bit 11 - A/D Conversion Start 1 = Conversion start. 0 = Conversion stopped and A/D converter enter idle state. ADST bit can be controlled by two sources: software write and external pin STADC. ADST is cleared to 0 by hardware automatically at the ends of single mode and single-cycle scan mode on specified channels. In continuous scan mode, A/D conversion is continuously performed sequentially until this bit is cleared to 0 or chip reset."]
    #[inline(always)]
    pub fn adst(&mut self) -> ADST_W {
        ADST_W { w: self }
    }
    #[doc = "Bit 31 - A/D differential input Mode Output Format This bit is only supported in Low Density. 1 = A/D Conversion result will be filled in RSLT at ADDRx registers with 2'complement format. 0 = A/D Conversion result will be filled in RSLT at ADDRx registers with unsigned format."]
    #[inline(always)]
    pub fn dmof(&mut self) -> DMOF_W {
        DMOF_W { w: self }
    }
}
