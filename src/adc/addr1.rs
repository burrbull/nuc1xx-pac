#[doc = "Reader of register ADDR1"]
pub type R = crate::R<u32, super::ADDR1>;
#[doc = "Reader of field `RSLT`"]
pub type RSLT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - A/D Conversion Result This field contains conversion result of ADC. For Medium density, RSLT\\[15:12\\]
always read as 0. For Low density, if DMOF bit (ADCR\\[31\\]) set to 0, 12 bits ADC conversion result with unsigned format will be filled in RSLT\\[11:0\\]
and zero will be filled in RSLT\\[15:12\\]. If DMOF bit (ADCR\\[31\\]) set to 1, 12 bits ADC conversion result with 2's complement format will be filled in RSLT\\[11:0\\]
and signed bits will be filled in RSLT\\[15:12\\]."]
    #[inline(always)]
    pub fn rslt(&self) -> RSLT_R {
        RSLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Over Run Flag 1 = Data in RSLT\\[11:0\\]
is overwrite. 0 = Data in RSLT\\[11:0\\]
is recent conversion result. If converted data in RSLT\\[11:0\\]
has not been read before new conversion result is loaded to this register, OVERRUN is set to 1 and previous conversion result is gone. It will be cleared by hardware after ADDR register is read."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Valid Flag 1 = Data in RSLT\\[11:0\\]
bits is valid. 0 = Data in RSLT\\[11:0\\]
bits is not valid. This bit is set to 1 when corresponding channel analog input conversion is completed and cleared by hardware after ADDR register is read."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
