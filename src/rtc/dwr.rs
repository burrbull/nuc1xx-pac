#[doc = "Reader of register DWR"]
pub type R = crate::R<u32, super::DWR>;
#[doc = "Writer for register DWR"]
pub type W = crate::W<u32, super::DWR>;
#[doc = "Register DWR `reset()`'s with value 0x06"]
impl crate::ResetValue for super::DWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `DWR`"]
pub type DWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DWR`"]
pub struct DWR_W<'a> {
    w: &'a mut W,
}
impl<'a> DWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Day of the Week Register Value Day of the Week 0 Sunday 1 Monday 2 Tuesday 3 Wednesday 4 Thursday 5 Friday 6 Saturday"]
    #[inline(always)]
    pub fn dwr(&self) -> DWR_R {
        DWR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of the Week Register Value Day of the Week 0 Sunday 1 Monday 2 Tuesday 3 Wednesday 4 Thursday 5 Friday 6 Saturday"]
    #[inline(always)]
    pub fn dwr(&mut self) -> DWR_W {
        DWR_W { w: self }
    }
}
