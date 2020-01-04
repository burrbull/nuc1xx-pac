#[doc = "Reader of register REGWRPROT"]
pub type R = crate::R<u32, super::REGWRPROT>;
#[doc = "Writer for register REGWRPROT"]
pub type W = crate::W<u32, super::REGWRPROT>;
#[doc = "Register REGWRPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::REGWRPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGPROTDIS`"]
pub type REGPROTDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGWRPROT`"]
pub struct REGWRPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> REGWRPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Protection Disable Index (Read only) 1 = Write-Protection is disabled for writing protected registers 0 = Write-Protection is enabled for writing protected registers. Any write to the protected register is ignored. The Protected registers are: IPRST1: address 0x5000_0008 BODCR: address 0x5000_0018 PORCR: address 0x5000_0024 PWRCON: address 0x5000_0200 (bit\\[6\\]
is not protected for power wake-up interrupt clear) APBCLK bit\\[0\\]: address 0x5000_0208 (bit\\[0\\]
is watchdog timer clock enable) CLK_SEL0: address 0x5000_0210 (for HCLK and CPU STCLK clock source select) CLK_SEL1 bit\\[1:0\\]: address 0x5000_0214 (for watch dog clock source select) ISPCON: address 0x5000_C000 (Flash ISP Control register) WTCR: address 0x4000_4000 FATCON: address 0x5000_C018"]
    #[inline(always)]
    pub fn regprotdis(&self) -> REGPROTDIS_R {
        REGPROTDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Write-Protection Code (Write only) Some write-protected registers have to be disabled the protected function by writing the sequence value \"59h\", \"16h\", \"88h\" to this field. After this sequence is completed, the REGPROTDIS bit will be set to 1 and write-protected registers can be normal write."]
    #[inline(always)]
    pub fn regwrprot(&mut self) -> REGWRPROT_W {
        REGWRPROT_W { w: self }
    }
}
