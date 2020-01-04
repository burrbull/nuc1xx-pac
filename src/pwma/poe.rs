#[doc = "Reader of register POE"]
pub type R = crate::R<u32, super::POE>;
#[doc = "Writer for register POE"]
pub type W = crate::W<u32, super::POE>;
#[doc = "Register POE `reset()`'s with value 0"]
impl crate::ResetValue for super::POE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM0`"]
pub type PWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM0`"]
pub struct PWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_W<'a> {
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
#[doc = "Reader of field `PWM1`"]
pub type PWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM1`"]
pub struct PWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_W<'a> {
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
#[doc = "Reader of field `PWM2`"]
pub type PWM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM2`"]
pub struct PWM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_W<'a> {
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
#[doc = "Reader of field `PWM3`"]
pub type PWM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM3`"]
pub struct PWM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3_W<'a> {
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
    #[doc = "Bit 0 - PWM Channel 0 Output Enable Register 1 = Enable PWM channel 0 output to pin 0 = Disable PWM channel 0 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Channel 1 Output Enable Register 1 = Enable PWM channel 1 output to pin 0 = Disable PWM channel 1 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Channel 2 Output Enable Register 1 = Enable PWM channel 2 output to pin 0 = Disable PWM channel 2 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm2(&self) -> PWM2_R {
        PWM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Channel 3 Output Enable Register 1 = Enable PWM channel 3 output to pin 0 = Disable PWM channel 3 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm3(&self) -> PWM3_R {
        PWM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Channel 0 Output Enable Register 1 = Enable PWM channel 0 output to pin 0 = Disable PWM channel 0 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm0(&mut self) -> PWM0_W {
        PWM0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Channel 1 Output Enable Register 1 = Enable PWM channel 1 output to pin 0 = Disable PWM channel 1 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm1(&mut self) -> PWM1_W {
        PWM1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Channel 2 Output Enable Register 1 = Enable PWM channel 2 output to pin 0 = Disable PWM channel 2 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm2(&mut self) -> PWM2_W {
        PWM2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Channel 3 Output Enable Register 1 = Enable PWM channel 3 output to pin 0 = Disable PWM channel 3 output to pin Note: The corresponding GPIO pin also must be switched to PWM function."]
    #[inline(always)]
    pub fn pwm3(&mut self) -> PWM3_W {
        PWM3_W { w: self }
    }
}
