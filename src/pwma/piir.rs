#[doc = "Reader of register PIIR"]
pub type R = crate::R<u32, super::PIIR>;
#[doc = "Writer for register PIIR"]
pub type W = crate::W<u32, super::PIIR>;
#[doc = "Register PIIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PIIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWMIF0`"]
pub type PWMIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIF0`"]
pub struct PWMIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIF0_W<'a> {
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
#[doc = "Reader of field `PWMIF1`"]
pub type PWMIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIF1`"]
pub struct PWMIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIF1_W<'a> {
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
#[doc = "Reader of field `PWMIF2`"]
pub type PWMIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIF2`"]
pub struct PWMIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIF2_W<'a> {
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
#[doc = "Reader of field `PWMIF3`"]
pub type PWMIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIF3`"]
pub struct PWMIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIF3_W<'a> {
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
    #[doc = "Bit 0 - PWM Channel 0 Interrupt Status Flag is set by hardware when PWM0 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif0(&self) -> PWMIF0_R {
        PWMIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Channel 1 Interrupt Status Flag is set by hardware when PWM1 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif1(&self) -> PWMIF1_R {
        PWMIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Channel 2 Interrupt Status Flag is set by hardware when PWM2 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif2(&self) -> PWMIF2_R {
        PWMIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Channel 3 Interrupt Status Flag is set by hardware when PWM3 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif3(&self) -> PWMIF3_R {
        PWMIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Channel 0 Interrupt Status Flag is set by hardware when PWM0 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif0(&mut self) -> PWMIF0_W {
        PWMIF0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Channel 1 Interrupt Status Flag is set by hardware when PWM1 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif1(&mut self) -> PWMIF1_W {
        PWMIF1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Channel 2 Interrupt Status Flag is set by hardware when PWM2 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif2(&mut self) -> PWMIF2_W {
        PWMIF2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Channel 3 Interrupt Status Flag is set by hardware when PWM3 down counter reaches zero, software can write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn pwmif3(&mut self) -> PWMIF3_W {
        PWMIF3_W { w: self }
    }
}
