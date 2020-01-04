#[doc = "Reader of register PORCR"]
pub type R = crate::R<u32, super::PORCR>;
#[doc = "Writer for register PORCR"]
pub type W = crate::W<u32, super::PORCR>;
#[doc = "Register PORCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PORCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POR_DIS_CODE`"]
pub type POR_DIS_CODE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POR_DIS_CODE`"]
pub struct POR_DIS_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_DIS_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The register is used for the Power-On-Reset enable control When power on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field. The POR function will be active again when this field is set to another value or chip is reset by other reset source, including: /RESET, Watch dog, LVR reset, BOD reset, ICE reset command and the software-chip reset function This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn por_dis_code(&self) -> POR_DIS_CODE_R {
        POR_DIS_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register is used for the Power-On-Reset enable control When power on, the POR circuit generates a reset signal to reset the whole chip function, but noise on the power may cause the POR active again. User can disable internal POR circuit to avoid unpredictable noise to cause chip reset by writing 0x5AA5 to this field. The POR function will be active again when this field is set to another value or chip is reset by other reset source, including: /RESET, Watch dog, LVR reset, BOD reset, ICE reset command and the software-chip reset function This bit is the protected bit. It means programming this needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100."]
    #[inline(always)]
    pub fn por_dis_code(&mut self) -> POR_DIS_CODE_W {
        POR_DIS_CODE_W { w: self }
    }
}
