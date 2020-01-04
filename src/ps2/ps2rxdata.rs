#[doc = "Reader of register PS2RXDATA"]
pub type R = crate::R<u32, super::PS2RXDATA>;
#[doc = "Writer for register PS2RXDATA"]
pub type W = crate::W<u32, super::PS2RXDATA>;
#[doc = "Register PS2RXDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::PS2RXDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PS2RXDATA`"]
pub type PS2RXDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received Data For host to device communication, after acknowledge bit is sent, the received data is copied from receive shift register to PS2RXDATA register. CPU must read this register before next byte reception complete, otherwise the data will be overwritten and RXOVF bit in PS2STATUS\\[6\\]
will be set to 1."]
    #[inline(always)]
    pub fn ps2rxdata(&self) -> PS2RXDATA_R {
        PS2RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
