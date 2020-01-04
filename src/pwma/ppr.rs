#[doc = "Reader of register PPR"]
pub type R = crate::R<u32, super::PPR>;
#[doc = "Writer for register PPR"]
pub type W = crate::W<u32, super::PPR>;
#[doc = "Register PPR `reset()`'s with value 0"]
impl crate::ResetValue for super::PPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CP01`"]
pub type CP01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CP01`"]
pub struct CP01_W<'a> {
    w: &'a mut W,
}
impl<'a> CP01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CP23`"]
pub type CP23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CP23`"]
pub struct CP23_W<'a> {
    w: &'a mut W,
}
impl<'a> CP23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DZI01`"]
pub type DZI01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DZI01`"]
pub struct DZI01_W<'a> {
    w: &'a mut W,
}
impl<'a> DZI01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DZI23`"]
pub type DZI23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DZI23`"]
pub struct DZI23_W<'a> {
    w: &'a mut W,
}
impl<'a> DZI23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock prescaler 0 (PWM-timer 0 & 1 for group A and PWM-timer 4 & 5 for group B) Clock input is divided by (CP01 + 1) before it is fed to the corresponding PWM-timer If CP01=0, then the clock prescaler 0 output clock will be stopped. So corresponding PWM-timer will be stopped also."]
    #[inline(always)]
    pub fn cp01(&self) -> CP01_R {
        CP01_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock prescaler 2 (PWM-timer2 & 3 for group A and PWM-timer 6 & 7 for group B) Clock input is divided by (CP23 + 1) before it is fed to the corresponding PWM-timer. If CP23=0, then the clock prescaler 2 output clock will be stopped. So corresponding PWM-timer will be stopped also."]
    #[inline(always)]
    pub fn cp23(&self) -> CP23_R {
        CP23_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Dead Zone Interval for Pair of Channel 0 and Channel 1 (PWM0 and PWM1 pair for PWM group A, PWM4 and PWM5 pair for PWM group B) These 8 bits determine dead zone length. The unit time of dead zone length is received from corresponding CSR bits."]
    #[inline(always)]
    pub fn dzi01(&self) -> DZI01_R {
        DZI01_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead Zone Interval for Pair of Channel2 and Channel3 (PWM2 and PWM3 pair for PWM group A, PWM6 and PWM7 pair for PWM group B) These 8 bits determine dead zone length. The unit time of dead zone length is received from corresponding CSR bits."]
    #[inline(always)]
    pub fn dzi23(&self) -> DZI23_R {
        DZI23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescaler 0 (PWM-timer 0 & 1 for group A and PWM-timer 4 & 5 for group B) Clock input is divided by (CP01 + 1) before it is fed to the corresponding PWM-timer If CP01=0, then the clock prescaler 0 output clock will be stopped. So corresponding PWM-timer will be stopped also."]
    #[inline(always)]
    pub fn cp01(&mut self) -> CP01_W {
        CP01_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock prescaler 2 (PWM-timer2 & 3 for group A and PWM-timer 6 & 7 for group B) Clock input is divided by (CP23 + 1) before it is fed to the corresponding PWM-timer. If CP23=0, then the clock prescaler 2 output clock will be stopped. So corresponding PWM-timer will be stopped also."]
    #[inline(always)]
    pub fn cp23(&mut self) -> CP23_W {
        CP23_W { w: self }
    }
    #[doc = "Bits 16:23 - Dead Zone Interval for Pair of Channel 0 and Channel 1 (PWM0 and PWM1 pair for PWM group A, PWM4 and PWM5 pair for PWM group B) These 8 bits determine dead zone length. The unit time of dead zone length is received from corresponding CSR bits."]
    #[inline(always)]
    pub fn dzi01(&mut self) -> DZI01_W {
        DZI01_W { w: self }
    }
    #[doc = "Bits 24:31 - Dead Zone Interval for Pair of Channel2 and Channel3 (PWM2 and PWM3 pair for PWM group A, PWM6 and PWM7 pair for PWM group B) These 8 bits determine dead zone length. The unit time of dead zone length is received from corresponding CSR bits."]
    #[inline(always)]
    pub fn dzi23(&mut self) -> DZI23_W {
        DZI23_W { w: self }
    }
}
