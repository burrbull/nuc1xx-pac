#[doc = "Reader of register NVIC_IPR0"]
pub type R = crate::R<u32, super::NVIC_IPR0>;
#[doc = "Writer for register NVIC_IPR0"]
pub type W = crate::W<u32, super::NVIC_IPR0>;
#[doc = "Register NVIC_IPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_0`"]
pub type PRI_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_0`"]
pub struct PRI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_1`"]
pub type PRI_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_1`"]
pub struct PRI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_2`"]
pub type PRI_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_2`"]
pub struct PRI_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_3`"]
pub type PRI_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_3`"]
pub struct PRI_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Priority of IRQ0 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_0(&self) -> PRI_0_R {
        PRI_0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of IRQ1 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_1(&self) -> PRI_1_R {
        PRI_1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of IRQ2 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_2(&self) -> PRI_2_R {
        PRI_2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Priority of IRQ3 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_3(&self) -> PRI_3_R {
        PRI_3_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of IRQ0 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_0(&mut self) -> PRI_0_W {
        PRI_0_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of IRQ1 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_1(&mut self) -> PRI_1_W {
        PRI_1_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of IRQ2 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_2(&mut self) -> PRI_2_W {
        PRI_2_W { w: self }
    }
    #[doc = "Bits 30:31 - Priority of IRQ3 \"0\" denotes the highest priority and \"3\" denotes lowest priority"]
    #[inline(always)]
    pub fn pri_3(&mut self) -> PRI_3_W {
        PRI_3_W { w: self }
    }
}
