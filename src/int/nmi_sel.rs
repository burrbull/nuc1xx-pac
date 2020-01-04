#[doc = "Reader of register NMI_SEL"]
pub type R = crate::R<u32, super::NMI_SEL>;
#[doc = "Writer for register NMI_SEL"]
pub type W = crate::W<u32, super::NMI_SEL>;
#[doc = "Register NMI_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::NMI_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMI_SEL`"]
pub type NMI_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NMI_SEL`"]
pub struct NMI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - The NMI interrupt to Cortex-M0 can be selected from one of the peripheral interrupt by setting NMI_SEL."]
    #[inline(always)]
    pub fn nmi_sel(&self) -> NMI_SEL_R {
        NMI_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The NMI interrupt to Cortex-M0 can be selected from one of the peripheral interrupt by setting NMI_SEL."]
    #[inline(always)]
    pub fn nmi_sel(&mut self) -> NMI_SEL_W {
        NMI_SEL_W { w: self }
    }
}
