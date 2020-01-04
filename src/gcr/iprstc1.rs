#[doc = "Reader of register IPRSTC1"]
pub type R = crate::R<u32, super::IPRSTC1>;
#[doc = "Writer for register IPRSTC1"]
pub type W = crate::W<u32, super::IPRSTC1>;
#[doc = "Register IPRSTC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPRSTC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHIP_RST`"]
pub type CHIP_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHIP_RST`"]
pub struct CHIP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_RST_W<'a> {
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
#[doc = "Reader of field `CPU_RST`"]
pub type CPU_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_RST`"]
pub struct CPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RST_W<'a> {
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
#[doc = "Reader of field `PDMA_RST`"]
pub type PDMA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMA_RST`"]
pub struct PDMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMA_RST_W<'a> {
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
#[doc = "Reader of field `EBI_RST`"]
pub type EBI_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_RST`"]
pub struct EBI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_RST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CHIP one shot reset (write-protection bit) Setting this bit will reset the whole chip, including CPU kernel and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles. The CHIP_RST is same as the POR reset, all the chip controllers is reset and the chip setting from flash are also reload. About the difference between CHIP_RST and SYSRESETREQ, please refer to section 5.2.2 of TRM. This bit is the protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = CHIP one shot reset 0 = CHIP normal operation"]
    #[inline(always)]
    pub fn chip_rst(&self) -> CHIP_RST_R {
        CHIP_RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU kernel one shot reset (write-protection bit) Setting this bit will only reset the CPU kernel and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = CPU one shot reset 0 = CPU normal operation"]
    #[inline(always)]
    pub fn cpu_rst(&self) -> CPU_RST_R {
        CPU_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDMA Controller Reset (write-protection bit in NUC100/NUC120/NUC130/NUC140 Low Density and NUC101) Setting this bit to 1 will generate a reset signal to the PDMA. User need to set this bit to 0 to release from reset state This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100 1 = PDMA controller reset 0 = PDMA controller normal operation"]
    #[inline(always)]
    pub fn pdma_rst(&self) -> PDMA_RST_R {
        PDMA_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EBI Controller Reset (write-protection bit in NUC100/NUC120/NUC130/NUC140 Low Density 64-pin package) Set this bit to 1 will generate a reset signal to the EBI. User need to set this bit to 0 to release from the reset state. This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = EBI controller reset 0 = EBI controller normal operation"]
    #[inline(always)]
    pub fn ebi_rst(&self) -> EBI_RST_R {
        EBI_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHIP one shot reset (write-protection bit) Setting this bit will reset the whole chip, including CPU kernel and all peripherals, and this bit will automatically return to 0 after the 2 clock cycles. The CHIP_RST is same as the POR reset, all the chip controllers is reset and the chip setting from flash are also reload. About the difference between CHIP_RST and SYSRESETREQ, please refer to section 5.2.2 of TRM. This bit is the protected bit. It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = CHIP one shot reset 0 = CHIP normal operation"]
    #[inline(always)]
    pub fn chip_rst(&mut self) -> CHIP_RST_W {
        CHIP_RST_W { w: self }
    }
    #[doc = "Bit 1 - CPU kernel one shot reset (write-protection bit) Setting this bit will only reset the CPU kernel and Flash Memory Controller(FMC), and this bit will automatically return to 0 after the 2 clock cycles This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = CPU one shot reset 0 = CPU normal operation"]
    #[inline(always)]
    pub fn cpu_rst(&mut self) -> CPU_RST_W {
        CPU_RST_W { w: self }
    }
    #[doc = "Bit 2 - PDMA Controller Reset (write-protection bit in NUC100/NUC120/NUC130/NUC140 Low Density and NUC101) Setting this bit to 1 will generate a reset signal to the PDMA. User need to set this bit to 0 to release from reset state This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100 1 = PDMA controller reset 0 = PDMA controller normal operation"]
    #[inline(always)]
    pub fn pdma_rst(&mut self) -> PDMA_RST_W {
        PDMA_RST_W { w: self }
    }
    #[doc = "Bit 3 - EBI Controller Reset (write-protection bit in NUC100/NUC120/NUC130/NUC140 Low Density 64-pin package) Set this bit to 1 will generate a reset signal to the EBI. User need to set this bit to 0 to release from the reset state. This bit is the protected bit, It means programming this bit needs to write \"59h\", \"16h\", \"88h\" to address 0x5000_0100 to disable register protection. Reference the register REGWRPROT at address GCR_BA+0x100. 1 = EBI controller reset 0 = EBI controller normal operation"]
    #[inline(always)]
    pub fn ebi_rst(&mut self) -> EBI_RST_W {
        EBI_RST_W { w: self }
    }
}
