#[doc = "Reader of register I2CDAT"]
pub type R = crate::R<u32, super::I2CDAT>;
#[doc = "Writer for register I2CDAT"]
pub type W = crate::W<u32, super::I2CDAT>;
#[doc = "Register I2CDAT `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CDAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CDAT`"]
pub type I2CDAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2CDAT`"]
pub struct I2CDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Data Register Bit\\[7:0\\]
is located with the 8-bit transferred data of I2C serial port."]
    #[inline(always)]
    pub fn i2cdat(&self) -> I2CDAT_R {
        I2CDAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Data Register Bit\\[7:0\\]
is located with the 8-bit transferred data of I2C serial port."]
    #[inline(always)]
    pub fn i2cdat(&mut self) -> I2CDAT_W {
        I2CDAT_W { w: self }
    }
}
