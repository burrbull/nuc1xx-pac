#[doc = "Reader of register PIER"]
pub type R = crate::R<u32, super::PIER>;
#[doc = "Writer for register PIER"]
pub type W = crate::W<u32, super::PIER>;
#[doc = "Register PIER `reset()`'s with value 0"]
impl crate::ResetValue for super::PIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWMIE0`"]
pub type PWMIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIE0`"]
pub struct PWMIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIE0_W<'a> {
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
#[doc = "Reader of field `PWMIE1`"]
pub type PWMIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIE1`"]
pub struct PWMIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIE1_W<'a> {
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
#[doc = "Reader of field `PWMIE2`"]
pub type PWMIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIE2`"]
pub struct PWMIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIE2_W<'a> {
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
#[doc = "Reader of field `PWMIE3`"]
pub type PWMIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMIE3`"]
pub struct PWMIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMIE3_W<'a> {
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
    #[doc = "Bit 0 - PWM Channel 0 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie0(&self) -> PWMIE0_R {
        PWMIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM Channel 1 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie1(&self) -> PWMIE1_R {
        PWMIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Channel 2 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie2(&self) -> PWMIE2_R {
        PWMIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Channel 3 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie3(&self) -> PWMIE3_R {
        PWMIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Channel 0 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie0(&mut self) -> PWMIE0_W {
        PWMIE0_W { w: self }
    }
    #[doc = "Bit 1 - PWM Channel 1 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie1(&mut self) -> PWMIE1_W {
        PWMIE1_W { w: self }
    }
    #[doc = "Bit 2 - PWM Channel 2 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie2(&mut self) -> PWMIE2_W {
        PWMIE2_W { w: self }
    }
    #[doc = "Bit 3 - PWM Channel 3 Interrupt Enable 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn pwmie3(&mut self) -> PWMIE3_W {
        PWMIE3_W { w: self }
    }
}
