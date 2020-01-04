#[doc = "Reader of register NVIC_IPR7"]
pub type R = crate::R<u32, super::NVIC_IPR7>;
#[doc = "Writer for register NVIC_IPR7"]
pub type W = crate::W<u32, super::NVIC_IPR7>;
#[doc = "Register NVIC_IPR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_28`"]
pub type PRI_28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_28`"]
pub struct PRI_28_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_29`"]
pub type PRI_29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_29`"]
pub struct PRI_29_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_30`"]
pub type PRI_30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_30`"]
pub struct PRI_30_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_31`"]
pub type PRI_31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_31`"]
pub struct PRI_31_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Priority of IRQ28 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_28(&self) -> PRI_28_R {
        PRI_28_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of IRQ29 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_29(&self) -> PRI_29_R {
        PRI_29_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of IRQ30 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_30(&self) -> PRI_30_R {
        PRI_30_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Priority of IRQ31 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_31(&self) -> PRI_31_R {
        PRI_31_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of IRQ28 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_28(&mut self) -> PRI_28_W {
        PRI_28_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of IRQ29 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_29(&mut self) -> PRI_29_W {
        PRI_29_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of IRQ30 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_30(&mut self) -> PRI_30_W {
        PRI_30_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority of IRQ31 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_31(&mut self) -> PRI_31_W {
        PRI_31_W { w: self }
    }
}
