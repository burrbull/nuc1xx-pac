#[doc = "Reader of register USB_BUFSEG2"]
pub type R = crate::R<u32, super::USB_BUFSEG2>;
#[doc = "Writer for register USB_BUFSEG2"]
pub type W = crate::W<u32, super::USB_BUFSEG2>;
#[doc = "Register USB_BUFSEG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_BUFSEG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFSEG2`"]
pub type BUFSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUFSEG2`"]
pub struct BUFSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | (((value as u32) & 0x3f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - It is used to indicate the offset address for each endpoint with the USB SRAM starting address. The effective starting address of the endpoint is: USB_SRAM address + { BUFSEG2\\[8:3\\], 3'b000} Where the USB_SRAM address = 0x40060100h. Refer to section 5.4.4.7 for the endpoint SRAM structure and its description."]
    #[inline(always)]
    pub fn bufseg2(&self) -> BUFSEG2_R {
        BUFSEG2_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:8 - It is used to indicate the offset address for each endpoint with the USB SRAM starting address. The effective starting address of the endpoint is: USB_SRAM address + { BUFSEG2\\[8:3\\], 3'b000} Where the USB_SRAM address = 0x40060100h. Refer to section 5.4.4.7 for the endpoint SRAM structure and its description."]
    #[inline(always)]
    pub fn bufseg2(&mut self) -> BUFSEG2_W {
        BUFSEG2_W { w: self }
    }
}
