#[doc = "Reader of register ALT_MFP"]
pub type R = crate::R<u32, super::ALT_MFP>;
#[doc = "Writer for register ALT_MFP"]
pub type W = crate::W<u32, super::ALT_MFP>;
#[doc = "Register ALT_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::ALT_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PB10_S01`"]
pub type PB10_S01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB10_S01`"]
pub struct PB10_S01_W<'a> {
    w: &'a mut W,
}
impl<'a> PB10_S01_W<'a> {
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
#[doc = "Reader of field `PB9_S11`"]
pub type PB9_S11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB9_S11`"]
pub struct PB9_S11_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_S11_W<'a> {
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
#[doc = "Reader of field `PA7_S21`"]
pub type PA7_S21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA7_S21`"]
pub struct PA7_S21_W<'a> {
    w: &'a mut W,
}
impl<'a> PA7_S21_W<'a> {
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
#[doc = "Reader of field `PB14_S31`"]
pub type PB14_S31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB14_S31`"]
pub struct PB14_S31_W<'a> {
    w: &'a mut W,
}
impl<'a> PB14_S31_W<'a> {
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
#[doc = "Reader of field `PB11_PWM4`"]
pub type PB11_PWM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB11_PWM4`"]
pub struct PB11_PWM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PB11_PWM4_W<'a> {
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
#[doc = "Reader of field `PC0_I2SLRCLK`"]
pub type PC0_I2SLRCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC0_I2SLRCLK`"]
pub struct PC0_I2SLRCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_I2SLRCLK_W<'a> {
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
#[doc = "Reader of field `PC1_I2SBCLK`"]
pub type PC1_I2SBCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC1_I2SBCLK`"]
pub struct PC1_I2SBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_I2SBCLK_W<'a> {
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
#[doc = "Reader of field `PC2_I2SDI`"]
pub type PC2_I2SDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC2_I2SDI`"]
pub struct PC2_I2SDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_I2SDI_W<'a> {
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
#[doc = "Reader of field `PC3_I2SDO`"]
pub type PC3_I2SDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC3_I2SDO`"]
pub struct PC3_I2SDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_I2SDO_W<'a> {
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
#[doc = "Reader of field `PA15_I2SMCLK`"]
pub type PA15_I2SMCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA15_I2SMCLK`"]
pub struct PA15_I2SMCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PA15_I2SMCLK_W<'a> {
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
#[doc = "Reader of field `PB12_CLKO`"]
pub type PB12_CLKO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB12_CLKO`"]
pub struct PB12_CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> PB12_CLKO_W<'a> {
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
#[doc = "Reader of field `EBI_EN`"]
pub type EBI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_EN`"]
pub struct EBI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_EN_W<'a> {
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
#[doc = "Reader of field `EBI_MCLK_EN`"]
pub type EBI_MCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_MCLK_EN`"]
pub struct EBI_MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_MCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EBI_nWRL_EN`"]
pub type EBI_NWRL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_nWRL_EN`"]
pub struct EBI_NWRL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_NWRL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `EBI_nWRH_EN`"]
pub type EBI_NWRH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI_nWRH_EN`"]
pub struct EBI_NWRH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_NWRH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EBI_HB_EN`"]
pub type EBI_HB_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EBI_HB_EN`"]
pub struct EBI_HB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_HB_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bits PB10_S01 and GPB_MFP10 determine the PB.10 function. PB10_S01 GPB_MFP\\[10\\]
PB.10 function x 0 GPIO 0 1 TM2 1 1 SPISS01 (SPI0)"]
    #[inline(always)]
    pub fn pb10_s01(&self) -> PB10_S01_R {
        PB10_S01_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bits PB9_S11 and GPB_MFP9 determine the PB.9 function. PB9_S11 GPB_MFP\\[9\\]
PB.9 function x 0 GPIO 0 1 TM1 1 1 SPISS11 (SPI1)"]
    #[inline(always)]
    pub fn pb9_s11(&self) -> PB9_S11_R {
        PB9_S11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bits PA7_S21, PA_MFP7 and EBI_EN (ALT_MFP\\[11\\])determine the PA.7 function. EBI_EN PA7_S21 GPA_MFP\\[7\\]
PA.7 function x x 0 GPIO 0 0 1 ADC7 (ADC) 0 1 1 SPISS21 (SPI2) 1 x 1 AD6 (EBI AD bus bit 6)"]
    #[inline(always)]
    pub fn pa7_s21(&self) -> PA7_S21_R {
        PA7_S21_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bits PB14_S31 and GPB_MFP14 determine the GPB14 function. PB14_S31 GPB_MFP\\[14\\]
PB.14 function x 0 GPIO 0 1 /INT0 1 1 SPISS31 (SPI3)"]
    #[inline(always)]
    pub fn pb14_s31(&self) -> PB14_S31_R {
        PB14_S31_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bits PB11_PWM4 and GPB_MFP\\[11\\]
determine the PB.11 function. PB11_PWM4 GPB_MFP\\[11\\]
PB.11 function x 0 GPIO 0 1 TM3 1 1 PWM4 (PWM)"]
    #[inline(always)]
    pub fn pb11_pwm4(&self) -> PB11_PWM4_R {
        PB11_PWM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bits PC0_I2SLRCLK and GPC_MFP\\[0\\]
determine the PC.0 function. PC0_I2SLRCLK GPC_MFP\\[0\\]
PC.0 function x 0 GPIO 0 1 SPISS00(SPI0) 1 1 I2SLRCLK (I2S)"]
    #[inline(always)]
    pub fn pc0_i2slrclk(&self) -> PC0_I2SLRCLK_R {
        PC0_I2SLRCLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bits PC1_I2SBCLK and GPC_MFP\\[1\\]
determine the PC.1 function. PC1_I2SBCLK GPC_MFP\\[1\\]
PC.1 function x 0 GPIO 0 1 SPICLK0 (SPI0) 1 1 I2SBLK (I2S)"]
    #[inline(always)]
    pub fn pc1_i2sbclk(&self) -> PC1_I2SBCLK_R {
        PC1_I2SBCLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bits PC2_I2SDI and GPC_MFP\\[2\\]
determine the PC.2 function. PC2_I2SDI GPC_MFP\\[2\\]
PC.2 function x 0 GPIO 0 1 MISO00 (SPI0) 1 1 I2SDI (I2S)"]
    #[inline(always)]
    pub fn pc2_i2sdi(&self) -> PC2_I2SDI_R {
        PC2_I2SDI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bits PC3_I2SDO and GPC_MFP\\[3\\]
determine the PC.3 function. PC3_I2SDO GPC_MFP\\[3\\]
PC.3 function x 0 GPIO 0 1 MOSI00 (SPI0) 1 1 I2SDO (I2S)"]
    #[inline(always)]
    pub fn pc3_i2sdo(&self) -> PC3_I2SDO_R {
        PC3_I2SDO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bits PA15_I2SMCLK and GPA_MFP\\[15\\]
determine the PA.15 function. PA15_I2SMCLK GPA_MFP\\[15\\]
PA.15 function x 0 GPIO 0 1 PWM3 (PWM) 1 1 I2SMCLK (I2S)"]
    #[inline(always)]
    pub fn pa15_i2smclk(&self) -> PA15_I2SMCLK_R {
        PA15_I2SMCLK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bits PB12_CLKO and GPB_MFP\\[12\\]
determine the PB.12 function. EBI_EN PB12_CLKO GPB_MFP\\[12\\]
PB.12 function x x 0 GPIO x 0 1 CPO0 (CMP) 0 1 1 CLKO (Clock Driver output) 1 1 1 AD0 (EBI AD bus bit 0)"]
    #[inline(always)]
    pub fn pb12_clko(&self) -> PB12_CLKO_R {
        PB12_CLKO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EBI_EN is use to switch GPIO function to EBI function (AD\\[15:0\\], ALE, RE, WE, CS, MCLK), it need additional registers EBI_EN\\[7:0\\]
and EBI_MCLK_EN for some GPIO to switch to EBI function(AD\\[15:8\\], MCLK)"]
    #[inline(always)]
    pub fn ebi_en(&self) -> EBI_EN_R {
        EBI_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bits EBI_MCLK_EN, EBI_EN and GPC_MFP\\[8\\]
determine the PC.8 function. EBI_MCLK_EN EBI_EN GPC_MFP\\[8\\]
PC.8 function x x 0 GPIO x 0 1 SPISS10 (SPI1) 0 1 1 SPISS10 (SPI1) 1 1 1 MCLK (EBI Clock output)"]
    #[inline(always)]
    pub fn ebi_mclk_en(&self) -> EBI_MCLK_EN_R {
        EBI_MCLK_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bits EBI_nWRL_EN, EBI_EN and GPB_MFP\\[2\\]
determine the PB.2 function. EBI_nWRL_EN EBI_EN GPB_MFP\\[2\\]
PB.2 function x x 0 GPIO x 0 1 RTS0 (UART0) 0 1 1 RTS0 (UART0) 1 1 1 nWRL (EBI write low byte enable)"]
    #[inline(always)]
    pub fn ebi_n_wrl_en(&self) -> EBI_NWRL_EN_R {
        EBI_NWRL_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bits EBI_nWRH_EN, EBI_EN and GPB_MFP\\[3\\]
determine the PB.3 function EBI_nWRH_EN EBI_EN GPB_MFP\\[3\\]
PB.3 function x x 0 GPIO x 0 1 CTS0 (UART0) 0 1 1 CTS0 (UART0) 1 1 1 nWRH (EBI write high byte enable)"]
    #[inline(always)]
    pub fn ebi_n_wrh_en(&self) -> EBI_NWRH_EN_R {
        EBI_NWRH_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - EBI_HB_EN is use to switch GPIO function to EBI address/data bus high byte (AD\\[15:8\\]), EBI_HB_EN, EBI_EN and corresponding GPx_MFP\\[y\\]
determine the Px.y function."]
    #[inline(always)]
    pub fn ebi_hb_en(&self) -> EBI_HB_EN_R {
        EBI_HB_EN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bits PB10_S01 and GPB_MFP10 determine the PB.10 function. PB10_S01 GPB_MFP\\[10\\]
PB.10 function x 0 GPIO 0 1 TM2 1 1 SPISS01 (SPI0)"]
    #[inline(always)]
    pub fn pb10_s01(&mut self) -> PB10_S01_W {
        PB10_S01_W { w: self }
    }
    #[doc = "Bit 1 - Bits PB9_S11 and GPB_MFP9 determine the PB.9 function. PB9_S11 GPB_MFP\\[9\\]
PB.9 function x 0 GPIO 0 1 TM1 1 1 SPISS11 (SPI1)"]
    #[inline(always)]
    pub fn pb9_s11(&mut self) -> PB9_S11_W {
        PB9_S11_W { w: self }
    }
    #[doc = "Bit 2 - Bits PA7_S21, PA_MFP7 and EBI_EN (ALT_MFP\\[11\\])determine the PA.7 function. EBI_EN PA7_S21 GPA_MFP\\[7\\]
PA.7 function x x 0 GPIO 0 0 1 ADC7 (ADC) 0 1 1 SPISS21 (SPI2) 1 x 1 AD6 (EBI AD bus bit 6)"]
    #[inline(always)]
    pub fn pa7_s21(&mut self) -> PA7_S21_W {
        PA7_S21_W { w: self }
    }
    #[doc = "Bit 3 - Bits PB14_S31 and GPB_MFP14 determine the GPB14 function. PB14_S31 GPB_MFP\\[14\\]
PB.14 function x 0 GPIO 0 1 /INT0 1 1 SPISS31 (SPI3)"]
    #[inline(always)]
    pub fn pb14_s31(&mut self) -> PB14_S31_W {
        PB14_S31_W { w: self }
    }
    #[doc = "Bit 4 - Bits PB11_PWM4 and GPB_MFP\\[11\\]
determine the PB.11 function. PB11_PWM4 GPB_MFP\\[11\\]
PB.11 function x 0 GPIO 0 1 TM3 1 1 PWM4 (PWM)"]
    #[inline(always)]
    pub fn pb11_pwm4(&mut self) -> PB11_PWM4_W {
        PB11_PWM4_W { w: self }
    }
    #[doc = "Bit 5 - Bits PC0_I2SLRCLK and GPC_MFP\\[0\\]
determine the PC.0 function. PC0_I2SLRCLK GPC_MFP\\[0\\]
PC.0 function x 0 GPIO 0 1 SPISS00(SPI0) 1 1 I2SLRCLK (I2S)"]
    #[inline(always)]
    pub fn pc0_i2slrclk(&mut self) -> PC0_I2SLRCLK_W {
        PC0_I2SLRCLK_W { w: self }
    }
    #[doc = "Bit 6 - Bits PC1_I2SBCLK and GPC_MFP\\[1\\]
determine the PC.1 function. PC1_I2SBCLK GPC_MFP\\[1\\]
PC.1 function x 0 GPIO 0 1 SPICLK0 (SPI0) 1 1 I2SBLK (I2S)"]
    #[inline(always)]
    pub fn pc1_i2sbclk(&mut self) -> PC1_I2SBCLK_W {
        PC1_I2SBCLK_W { w: self }
    }
    #[doc = "Bit 7 - Bits PC2_I2SDI and GPC_MFP\\[2\\]
determine the PC.2 function. PC2_I2SDI GPC_MFP\\[2\\]
PC.2 function x 0 GPIO 0 1 MISO00 (SPI0) 1 1 I2SDI (I2S)"]
    #[inline(always)]
    pub fn pc2_i2sdi(&mut self) -> PC2_I2SDI_W {
        PC2_I2SDI_W { w: self }
    }
    #[doc = "Bit 8 - Bits PC3_I2SDO and GPC_MFP\\[3\\]
determine the PC.3 function. PC3_I2SDO GPC_MFP\\[3\\]
PC.3 function x 0 GPIO 0 1 MOSI00 (SPI0) 1 1 I2SDO (I2S)"]
    #[inline(always)]
    pub fn pc3_i2sdo(&mut self) -> PC3_I2SDO_W {
        PC3_I2SDO_W { w: self }
    }
    #[doc = "Bit 9 - Bits PA15_I2SMCLK and GPA_MFP\\[15\\]
determine the PA.15 function. PA15_I2SMCLK GPA_MFP\\[15\\]
PA.15 function x 0 GPIO 0 1 PWM3 (PWM) 1 1 I2SMCLK (I2S)"]
    #[inline(always)]
    pub fn pa15_i2smclk(&mut self) -> PA15_I2SMCLK_W {
        PA15_I2SMCLK_W { w: self }
    }
    #[doc = "Bit 10 - Bits PB12_CLKO and GPB_MFP\\[12\\]
determine the PB.12 function. EBI_EN PB12_CLKO GPB_MFP\\[12\\]
PB.12 function x x 0 GPIO x 0 1 CPO0 (CMP) 0 1 1 CLKO (Clock Driver output) 1 1 1 AD0 (EBI AD bus bit 0)"]
    #[inline(always)]
    pub fn pb12_clko(&mut self) -> PB12_CLKO_W {
        PB12_CLKO_W { w: self }
    }
    #[doc = "Bit 11 - EBI_EN is use to switch GPIO function to EBI function (AD\\[15:0\\], ALE, RE, WE, CS, MCLK), it need additional registers EBI_EN\\[7:0\\]
and EBI_MCLK_EN for some GPIO to switch to EBI function(AD\\[15:8\\], MCLK)"]
    #[inline(always)]
    pub fn ebi_en(&mut self) -> EBI_EN_W {
        EBI_EN_W { w: self }
    }
    #[doc = "Bit 12 - Bits EBI_MCLK_EN, EBI_EN and GPC_MFP\\[8\\]
determine the PC.8 function. EBI_MCLK_EN EBI_EN GPC_MFP\\[8\\]
PC.8 function x x 0 GPIO x 0 1 SPISS10 (SPI1) 0 1 1 SPISS10 (SPI1) 1 1 1 MCLK (EBI Clock output)"]
    #[inline(always)]
    pub fn ebi_mclk_en(&mut self) -> EBI_MCLK_EN_W {
        EBI_MCLK_EN_W { w: self }
    }
    #[doc = "Bit 13 - Bits EBI_nWRL_EN, EBI_EN and GPB_MFP\\[2\\]
determine the PB.2 function. EBI_nWRL_EN EBI_EN GPB_MFP\\[2\\]
PB.2 function x x 0 GPIO x 0 1 RTS0 (UART0) 0 1 1 RTS0 (UART0) 1 1 1 nWRL (EBI write low byte enable)"]
    #[inline(always)]
    pub fn ebi_n_wrl_en(&mut self) -> EBI_NWRL_EN_W {
        EBI_NWRL_EN_W { w: self }
    }
    #[doc = "Bit 14 - Bits EBI_nWRH_EN, EBI_EN and GPB_MFP\\[3\\]
determine the PB.3 function EBI_nWRH_EN EBI_EN GPB_MFP\\[3\\]
PB.3 function x x 0 GPIO x 0 1 CTS0 (UART0) 0 1 1 CTS0 (UART0) 1 1 1 nWRH (EBI write high byte enable)"]
    #[inline(always)]
    pub fn ebi_n_wrh_en(&mut self) -> EBI_NWRH_EN_W {
        EBI_NWRH_EN_W { w: self }
    }
    #[doc = "Bits 16:23 - EBI_HB_EN is use to switch GPIO function to EBI address/data bus high byte (AD\\[15:8\\]), EBI_HB_EN, EBI_EN and corresponding GPx_MFP\\[y\\]
determine the Px.y function."]
    #[inline(always)]
    pub fn ebi_hb_en(&mut self) -> EBI_HB_EN_W {
        EBI_HB_EN_W { w: self }
    }
}
