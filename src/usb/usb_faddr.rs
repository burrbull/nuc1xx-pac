#[doc = "Reader of register USB_FADDR"]
pub type R = crate::R<u32, super::USB_FADDR>;
#[doc = "Writer for register USB_FADDR"]
pub type W = crate::W<u32, super::USB_FADDR>;
#[doc = "Register USB_FADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_FADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FADDR`"]
pub type FADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FADDR`"]
pub struct FADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Function Address of this USB device."]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address of this USB device."]
    #[inline(always)]
    pub fn faddr(&mut self) -> FADDR_W {
        FADDR_W { w: self }
    }
}
