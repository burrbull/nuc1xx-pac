#[doc = "Reader of register UA_THR"]
pub type R = crate::R<u32, super::UA_THR>;
#[doc = "Writer for register UA_THR"]
pub type W = crate::W<u32, super::UA_THR>;
#[doc = "Register UA_THR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `THR`"]
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:7 - Transmit Holding Register By writing to this register, the UART will send out an 8-bit data through the TX pin (LSB first)."]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
}
