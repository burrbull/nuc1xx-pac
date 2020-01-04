#[doc = "Reader of register DOUT[%s]"]
pub type R = crate::R<u32, super::DOUT>;
#[doc = "Writer for register DOUT[%s]"]
pub type W = crate::W<u32, super::DOUT>;
#[doc = "Register DOUT[%s]
`reset()`'s with value 0x01"]
impl crate::ResetValue for super::DOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "GPIOxx I/O Pin Bit Output Control Set this bit can control one GPIO pin output value For example: write GPIOx0_DOUT will reflect the written value to bit GPIOx_DOUT\\[0\\], read GPIOx0_DOUT will return the value of GPIOx_PIN\\[0\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT_A {
    #[doc = "0: Set corresponding GPIO bit to low"]
    LOW = 0,
    #[doc = "1: Set corresponding GPIO bit to high"]
    HIGH = 1,
}
impl From<DOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<bool, DOUT_A>;
impl DOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_A {
        match self.bits {
            false => DOUT_A::LOW,
            true => DOUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DOUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DOUT_A::HIGH
    }
}
#[doc = "Write proxy for field `DOUT`"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set corresponding GPIO bit to low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DOUT_A::LOW)
    }
    #[doc = "Set corresponding GPIO bit to high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DOUT_A::HIGH)
    }
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
impl R {
    #[doc = "Bit 0 - GPIOxx I/O Pin Bit Output Control Set this bit can control one GPIO pin output value For example: write GPIOx0_DOUT will reflect the written value to bit GPIOx_DOUT\\[0\\], read GPIOx0_DOUT will return the value of GPIOx_PIN\\[0\\]."]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOxx I/O Pin Bit Output Control Set this bit can control one GPIO pin output value For example: write GPIOx0_DOUT will reflect the written value to bit GPIOx_DOUT\\[0\\], read GPIOx0_DOUT will return the value of GPIOx_PIN\\[0\\]."]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W { w: self }
    }
}
