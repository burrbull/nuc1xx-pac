#[doc = "Reader of register NVIC_ISER"]
pub type R = crate::R<u32, super::NVIC_ISER>;
#[doc = "Writer for register NVIC_ISER"]
pub type W = crate::W<u32, super::NVIC_ISER>;
#[doc = "Register NVIC_ISER `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETENA`"]
pub type SETENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SETENA`"]
pub struct SETENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Enable one or more interrupts within a group of 32. Each bit represents an interrupt number from IRQ0 ~ IRQ31 (Vector number from 16 ~ 47). Writing 1 will enable the associated interrupt. Writing 0 has no effect. The register reads back with the current enable state."]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable one or more interrupts within a group of 32. Each bit represents an interrupt number from IRQ0 ~ IRQ31 (Vector number from 16 ~ 47). Writing 1 will enable the associated interrupt. Writing 0 has no effect. The register reads back with the current enable state."]
    #[inline(always)]
    pub fn setena(&mut self) -> SETENA_W {
        SETENA_W { w: self }
    }
}
