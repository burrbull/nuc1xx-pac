#[doc = "Reader of register I2CADM0"]
pub type R = crate::R<u32, super::I2CADM0>;
#[doc = "Writer for register I2CADM0"]
pub type W = crate::W<u32, super::I2CADM0>;
#[doc = "Register I2CADM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CADM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CADM`"]
pub type I2CADM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2CADM`"]
pub struct I2CADM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CADM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - I2C Address Mask register 1 = Mask enable (the received corresponding address bit is don't care.) 0 = Mask disable (the received corresponding register bit should be exact the same as address register.) I2C bus controllers support multiple address recognition with four address mask reg. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn i2cadm(&self) -> I2CADM_R {
        I2CADM_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - I2C Address Mask register 1 = Mask enable (the received corresponding address bit is don't care.) 0 = Mask disable (the received corresponding register bit should be exact the same as address register.) I2C bus controllers support multiple address recognition with four address mask reg. When the bit in the address mask register is set to one, it means the received corresponding address bit is don't-care. If the bit is set to zero, that means the received corresponding register bit should be exact the same as address register."]
    #[inline(always)]
    pub fn i2cadm(&mut self) -> I2CADM_W {
        I2CADM_W { w: self }
    }
}
