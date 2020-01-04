#[doc = "Reader of register NVIC_IPR2"]
pub type R = crate::R<u32, super::NVIC_IPR2>;
#[doc = "Writer for register NVIC_IPR2"]
pub type W = crate::W<u32, super::NVIC_IPR2>;
#[doc = "Register NVIC_IPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_8`"]
pub type PRI_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_8`"]
pub struct PRI_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_9`"]
pub type PRI_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_9`"]
pub struct PRI_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_10`"]
pub type PRI_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_10`"]
pub struct PRI_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
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
    #[doc = "Bits 6:7 - Priority of IRQ8 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_8(&self) -> PRI_8_R {
        PRI_8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of IRQ9 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_9(&self) -> PRI_9_R {
        PRI_9_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of IRQ10 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_10(&self) -> PRI_10_R {
        PRI_10_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Priority of IRQ11 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of IRQ8 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_8(&mut self) -> PRI_8_W {
        PRI_8_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of IRQ9 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_9(&mut self) -> PRI_9_W {
        PRI_9_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of IRQ10 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_10(&mut self) -> PRI_10_W {
        PRI_10_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority of IRQ11 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_11(&mut self) -> PRI_11_W {
        PRI_11_W { w: self }
    }
}
