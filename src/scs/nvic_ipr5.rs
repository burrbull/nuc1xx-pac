#[doc = "Reader of register NVIC_IPR5"]
pub type R = crate::R<u32, super::NVIC_IPR5>;
#[doc = "Writer for register NVIC_IPR5"]
pub type W = crate::W<u32, super::NVIC_IPR5>;
#[doc = "Register NVIC_IPR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_20`"]
pub type PRI_20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_20`"]
pub struct PRI_20_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_21`"]
pub type PRI_21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_21`"]
pub struct PRI_21_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_22`"]
pub type PRI_22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_22`"]
pub struct PRI_22_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_23`"]
pub type PRI_23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_23`"]
pub struct PRI_23_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Priority of IRQ20 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_20(&self) -> PRI_20_R {
        PRI_20_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of IRQ21 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_21(&self) -> PRI_21_R {
        PRI_21_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of IRQ22 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_22(&self) -> PRI_22_R {
        PRI_22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Priority of IRQ23 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_23(&self) -> PRI_23_R {
        PRI_23_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of IRQ20 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_20(&mut self) -> PRI_20_W {
        PRI_20_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of IRQ21 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_21(&mut self) -> PRI_21_W {
        PRI_21_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of IRQ22 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_22(&mut self) -> PRI_22_W {
        PRI_22_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority of IRQ23 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_23(&mut self) -> PRI_23_W {
        PRI_23_W { w: self }
    }
}
