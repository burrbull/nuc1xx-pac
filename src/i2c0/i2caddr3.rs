#[doc = "Reader of register I2CADDR3"]
pub type R = crate::R<u32, super::I2CADDR3>;
#[doc = "Writer for register I2CADDR3"]
pub type W = crate::W<u32, super::I2CADDR3>;
#[doc = "Register I2CADDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CADDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GC`"]
pub type GC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GC`"]
pub struct GC_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_W<'a> {
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
#[doc = "Reader of field `I2CADDR`"]
pub type I2CADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2CADDR`"]
pub struct I2CADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Call Function 0 = Disable General Call Function. 1 = Enable General Call Function."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - I2C Address register The content of this register is irrelevant when I2C is in master mode. In the slave mode, the seven most significant bits must be loaded with the MCU's own address. The I2C hardware will react if either of the address is matched."]
    #[inline(always)]
    pub fn i2caddr(&self) -> I2CADDR_R {
        I2CADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Function 0 = Disable General Call Function. 1 = Enable General Call Function."]
    #[inline(always)]
    pub fn gc(&mut self) -> GC_W {
        GC_W { w: self }
    }
    #[doc = "Bits 1:7 - I2C Address register The content of this register is irrelevant when I2C is in master mode. In the slave mode, the seven most significant bits must be loaded with the MCU's own address. The I2C hardware will react if either of the address is matched."]
    #[inline(always)]
    pub fn i2caddr(&mut self) -> I2CADDR_W {
        I2CADDR_W { w: self }
    }
}
