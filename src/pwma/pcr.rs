#[doc = "Reader of register PCR"]
pub type R = crate::R<u32, super::PCR>;
#[doc = "Writer for register PCR"]
pub type W = crate::W<u32, super::PCR>;
#[doc = "Register PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0EN`"]
pub type CH0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0EN`"]
pub struct CH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0EN_W<'a> {
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
#[doc = "Reader of field `CH0INV`"]
pub type CH0INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0INV`"]
pub struct CH0INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0INV_W<'a> {
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
#[doc = "Reader of field `CH0MOD`"]
pub type CH0MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0MOD`"]
pub struct CH0MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0MOD_W<'a> {
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
#[doc = "Reader of field `DZEN01`"]
pub type DZEN01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DZEN01`"]
pub struct DZEN01_W<'a> {
    w: &'a mut W,
}
impl<'a> DZEN01_W<'a> {
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
#[doc = "Reader of field `DZEN23`"]
pub type DZEN23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DZEN23`"]
pub struct DZEN23_W<'a> {
    w: &'a mut W,
}
impl<'a> DZEN23_W<'a> {
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
#[doc = "Reader of field `CH1EN`"]
pub type CH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1EN`"]
pub struct CH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1EN_W<'a> {
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
#[doc = "Reader of field `CH1INV`"]
pub type CH1INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1INV`"]
pub struct CH1INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1INV_W<'a> {
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
#[doc = "Reader of field `CH1MOD`"]
pub type CH1MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1MOD`"]
pub struct CH1MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1MOD_W<'a> {
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
#[doc = "Reader of field `CH2EN`"]
pub type CH2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2EN`"]
pub struct CH2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2EN_W<'a> {
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
#[doc = "Reader of field `CH2INV`"]
pub type CH2INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2INV`"]
pub struct CH2INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2INV_W<'a> {
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
#[doc = "Reader of field `CH2MOD`"]
pub type CH2MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2MOD`"]
pub struct CH2MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2MOD_W<'a> {
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
#[doc = "Reader of field `CH3EN`"]
pub type CH3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3EN`"]
pub struct CH3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH3INV`"]
pub type CH3INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3INV`"]
pub struct CH3INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CH3MOD`"]
pub type CH3MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3MOD`"]
pub struct CH3MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWM-Timer 0 Enable (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM-Timer 0 Output Inverter Enable (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch0inv(&self) -> CH0INV_R {
        CH0INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM-Timer 0 Auto-reload/One-Shot Mode (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR0 and CMR0 be clear."]
    #[inline(always)]
    pub fn ch0mod(&self) -> CH0MOD_R {
        CH0MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dead-Zone 0 Generator Enable (PWM0 and PWM1 pair for PWM group A, PWM4 and PWM5 pair for PWM group B) 1 = Enable 0 = Disable Note: When Dead-Zone Generator is enabled, the pair of PWM0 and PWM1 becomes a complementary pair for PWM group A and the pair of PWM4 and PWM5 becomes a complementary pair for PWM group B."]
    #[inline(always)]
    pub fn dzen01(&self) -> DZEN01_R {
        DZEN01_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dead-Zone 2 Generator Enable (PWM2 and PWM3 pair for PWM group A, PWM6 and PWM7 pair for PWM group B) 1 = Enable 0 = Disable Note: When Dead-Zone Generator is enabled, the pair of PWM2 and PWM3 becomes a complementary pair for PWM group A and the pair of PWM6 and PWM7 becomes a complementary pair for PWM group B."]
    #[inline(always)]
    pub fn dzen23(&self) -> DZEN23_R {
        DZEN23_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PWM-Timer 1 Enable (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM-Timer 1 Output Inverter Enable (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch1inv(&self) -> CH1INV_R {
        CH1INV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM-Timer 1 Auto-reload/One-Shot Mode (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Auto-load Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR1 and CMR1 be clear."]
    #[inline(always)]
    pub fn ch1mod(&self) -> CH1MOD_R {
        CH1MOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM-Timer 2 Enable (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch2en(&self) -> CH2EN_R {
        CH2EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PWM-Timer 2 Output Inverter Enable (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch2inv(&self) -> CH2INV_R {
        CH2INV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PWM-Timer 2 Auto-reload/One-Shot Mode (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR2 and CMR2 be clear."]
    #[inline(always)]
    pub fn ch2mod(&self) -> CH2MOD_R {
        CH2MOD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PWM-Timer 3 Enable (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch3en(&self) -> CH3EN_R {
        CH3EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PWM-Timer 3 Output Inverter Enable (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch3inv(&self) -> CH3INV_R {
        CH3INV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PWM-Timer 3 Auto-reload/One-Shot Mode (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR3 and CMR3 be clear."]
    #[inline(always)]
    pub fn ch3mod(&self) -> CH3MOD_R {
        CH3MOD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM-Timer 0 Enable (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W { w: self }
    }
    #[doc = "Bit 2 - PWM-Timer 0 Output Inverter Enable (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch0inv(&mut self) -> CH0INV_W {
        CH0INV_W { w: self }
    }
    #[doc = "Bit 3 - PWM-Timer 0 Auto-reload/One-Shot Mode (PWM timer 0 for group A and PWM timer 4 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR0 and CMR0 be clear."]
    #[inline(always)]
    pub fn ch0mod(&mut self) -> CH0MOD_W {
        CH0MOD_W { w: self }
    }
    #[doc = "Bit 4 - Dead-Zone 0 Generator Enable (PWM0 and PWM1 pair for PWM group A, PWM4 and PWM5 pair for PWM group B) 1 = Enable 0 = Disable Note: When Dead-Zone Generator is enabled, the pair of PWM0 and PWM1 becomes a complementary pair for PWM group A and the pair of PWM4 and PWM5 becomes a complementary pair for PWM group B."]
    #[inline(always)]
    pub fn dzen01(&mut self) -> DZEN01_W {
        DZEN01_W { w: self }
    }
    #[doc = "Bit 5 - Dead-Zone 2 Generator Enable (PWM2 and PWM3 pair for PWM group A, PWM6 and PWM7 pair for PWM group B) 1 = Enable 0 = Disable Note: When Dead-Zone Generator is enabled, the pair of PWM2 and PWM3 becomes a complementary pair for PWM group A and the pair of PWM6 and PWM7 becomes a complementary pair for PWM group B."]
    #[inline(always)]
    pub fn dzen23(&mut self) -> DZEN23_W {
        DZEN23_W { w: self }
    }
    #[doc = "Bit 8 - PWM-Timer 1 Enable (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W { w: self }
    }
    #[doc = "Bit 10 - PWM-Timer 1 Output Inverter Enable (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch1inv(&mut self) -> CH1INV_W {
        CH1INV_W { w: self }
    }
    #[doc = "Bit 11 - PWM-Timer 1 Auto-reload/One-Shot Mode (PWM timer 1 for group A and PWM timer 5 for group B) 1 = Auto-load Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR1 and CMR1 be clear."]
    #[inline(always)]
    pub fn ch1mod(&mut self) -> CH1MOD_W {
        CH1MOD_W { w: self }
    }
    #[doc = "Bit 16 - PWM-Timer 2 Enable (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch2en(&mut self) -> CH2EN_W {
        CH2EN_W { w: self }
    }
    #[doc = "Bit 18 - PWM-Timer 2 Output Inverter Enable (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch2inv(&mut self) -> CH2INV_W {
        CH2INV_W { w: self }
    }
    #[doc = "Bit 19 - PWM-Timer 2 Auto-reload/One-Shot Mode (PWM timer 2 for group A and PWM timer 6 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR2 and CMR2 be clear."]
    #[inline(always)]
    pub fn ch2mod(&mut self) -> CH2MOD_W {
        CH2MOD_W { w: self }
    }
    #[doc = "Bit 24 - PWM-Timer 3 Enable (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Enable corresponding PWM-Timer Start Run 0 = Stop corresponding PWM-Timer Running"]
    #[inline(always)]
    pub fn ch3en(&mut self) -> CH3EN_W {
        CH3EN_W { w: self }
    }
    #[doc = "Bit 26 - PWM-Timer 3 Output Inverter Enable (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Inverter enable 0 = Inverter disable"]
    #[inline(always)]
    pub fn ch3inv(&mut self) -> CH3INV_W {
        CH3INV_W { w: self }
    }
    #[doc = "Bit 27 - PWM-Timer 3 Auto-reload/One-Shot Mode (PWM timer 3 for group A and PWM timer 7 for group B) 1 = Auto-reload Mode 0 = One-Shot Mode Note: If there is a rising transition at this bit, it will cause CNR3 and CMR3 be clear."]
    #[inline(always)]
    pub fn ch3mod(&mut self) -> CH3MOD_W {
        CH3MOD_W { w: self }
    }
}
