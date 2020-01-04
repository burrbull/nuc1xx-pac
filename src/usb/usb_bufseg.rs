#[doc = "Reader of register USB_BUFSEG"]
pub type R = crate::R<u32, super::USB_BUFSEG>;
#[doc = "Writer for register USB_BUFSEG"]
pub type W = crate::W<u32, super::USB_BUFSEG>;
#[doc = "Register USB_BUFSEG `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_BUFSEG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFSEG`"]
pub type BUFSEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUFSEG`"]
pub struct BUFSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | (((value as u32) & 0x3f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - It is used to indicate the offset address for the Setup token with the USB SRAM starting address. The effective starting address is USB_SRAM address + { BUFSEG\\[8:3\\], 3'b000} Where the USB_SRAM address = 0x40060100h. Note: It is used for Setup token only."]
    #[inline(always)]
    pub fn bufseg(&self) -> BUFSEG_R {
        BUFSEG_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:8 - It is used to indicate the offset address for the Setup token with the USB SRAM starting address. The effective starting address is USB_SRAM address + { BUFSEG\\[8:3\\], 3'b000} Where the USB_SRAM address = 0x40060100h. Note: It is used for Setup token only."]
    #[inline(always)]
    pub fn bufseg(&mut self) -> BUFSEG_W {
        BUFSEG_W { w: self }
    }
}
