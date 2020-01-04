#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR0`"]
pub type CSR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSR0`"]
pub struct CSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CSR1`"]
pub type CSR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSR1`"]
pub struct CSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CSR2`"]
pub type CSR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSR2`"]
pub struct CSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CSR3`"]
pub type CSR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSR3`"]
pub struct CSR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PWM Timer 0 Clock Source Selection (PWM timer 0 for group A and PWM timer 4 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr0(&self) -> CSR0_R {
        CSR0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - PWM Timer 1 Clock Source Selection (PWM timer 1 for group A and PWM timer 5 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr1(&self) -> CSR1_R {
        CSR1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - PWM Timer 2 Clock Source Selection (PWM timer 2 for group A and PWM timer 6 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr2(&self) -> CSR2_R {
        CSR2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - PWM Timer 3 Clock Source Selection (PWM timer 3 for group A and PWM timer 7 for group B) Select clock input for PWM timer. CSR3 \\[14:12\\]
Input clock divided by 100 1 011 16 010 8 001 4 000 2"]
    #[inline(always)]
    pub fn csr3(&self) -> CSR3_R {
        CSR3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM Timer 0 Clock Source Selection (PWM timer 0 for group A and PWM timer 4 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr0(&mut self) -> CSR0_W {
        CSR0_W { w: self }
    }
    #[doc = "Bits 4:6 - PWM Timer 1 Clock Source Selection (PWM timer 1 for group A and PWM timer 5 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr1(&mut self) -> CSR1_W {
        CSR1_W { w: self }
    }
    #[doc = "Bits 8:10 - PWM Timer 2 Clock Source Selection (PWM timer 2 for group A and PWM timer 6 for group B) Select clock input for PWM timer. (Table is the same as CSR3)"]
    #[inline(always)]
    pub fn csr2(&mut self) -> CSR2_W {
        CSR2_W { w: self }
    }
    #[doc = "Bits 12:14 - PWM Timer 3 Clock Source Selection (PWM timer 3 for group A and PWM timer 7 for group B) Select clock input for PWM timer. CSR3 \\[14:12\\]
Input clock divided by 100 1 011 16 010 8 001 4 000 2"]
    #[inline(always)]
    pub fn csr3(&mut self) -> CSR3_W {
        CSR3_W { w: self }
    }
}
