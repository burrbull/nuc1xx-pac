#[doc = "Reader of register NVIC_IPR1"]
pub type R = crate::R<u32, super::NVIC_IPR1>;
#[doc = "Writer for register NVIC_IPR1"]
pub type W = crate::W<u32, super::NVIC_IPR1>;
#[doc = "Register NVIC_IPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_4`"]
pub type PRI_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_4`"]
pub struct PRI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_5`"]
pub type PRI_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_5`"]
pub struct PRI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_6`"]
pub type PRI_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_6`"]
pub struct PRI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_7`"]
pub type PRI_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_7`"]
pub struct PRI_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Priority of IRQ4 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of IRQ5 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of IRQ6 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Priority of IRQ7 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_7(&self) -> PRI_7_R {
        PRI_7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of IRQ4 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_4(&mut self) -> PRI_4_W {
        PRI_4_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of IRQ5 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_5(&mut self) -> PRI_5_W {
        PRI_5_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of IRQ6 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_6(&mut self) -> PRI_6_W {
        PRI_6_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority of IRQ7 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_7(&mut self) -> PRI_7_W {
        PRI_7_W { w: self }
    }
}
