#[doc = "Reader of register SHPR2"]
pub type R = crate::R<u32, super::SHPR2>;
#[doc = "Writer for register SHPR2"]
pub type W = crate::W<u32, super::SHPR2>;
#[doc = "Register SHPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_11`"]
pub type PRI_11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_11`"]
pub struct PRI_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of system handler 11 SVCall \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of system handler 11 SVCall \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_11(&mut self) -> PRI_11_W {
        PRI_11_W { w: self }
    }
}
