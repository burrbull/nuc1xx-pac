#[doc = "Reader of register DOUT10"]
pub type R = crate::R<u32, super::DOUT10>;
#[doc = "Writer for register DOUT10"]
pub type W = crate::W<u32, super::DOUT10>;
#[doc = "Register DOUT10 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DOUT10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT`"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
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
    #[doc = "Bit 0 - GPIOxx I/O Pin Bit Output Control Set this bit can control one GPIO pin output value 1 = Set corresponding GPIO bit to high 0 = Set corresponding GPIO bit to low For example: write GPIOx0_DOUT will reflect the written value to bit GPIOx_DOUT\\[0\\], read GPIOx0_DOUT will return the valur of GPIOx_PIN\\[0\\]."]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOxx I/O Pin Bit Output Control Set this bit can control one GPIO pin output value 1 = Set corresponding GPIO bit to high 0 = Set corresponding GPIO bit to low For example: write GPIOx0_DOUT will reflect the written value to bit GPIOx_DOUT\\[0\\], read GPIOx0_DOUT will return the valur of GPIOx_PIN\\[0\\]."]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W { w: self }
    }
}
