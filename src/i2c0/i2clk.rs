#[doc = "Reader of register I2CLK"]
pub type R = crate::R<u32, super::I2CLK>;
#[doc = "Writer for register I2CLK"]
pub type W = crate::W<u32, super::I2CLK>;
#[doc = "Register I2CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CLK`"]
pub type I2CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2CLK`"]
pub struct I2CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Clock Divided Register The I2C clock rate bits: Data Baud Rate of I2C = system clock /(4x(I2CLK+1))."]
    #[inline(always)]
    pub fn i2clk(&self) -> I2CLK_R {
        I2CLK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Clock Divided Register The I2C clock rate bits: Data Baud Rate of I2C = system clock /(4x(I2CLK+1))."]
    #[inline(always)]
    pub fn i2clk(&mut self) -> I2CLK_W {
        I2CLK_W { w: self }
    }
}
