#[doc = "Reader of register PMD"]
pub type R = crate::R<u32, super::PMD>;
#[doc = "Writer for register PMD"]
pub type W = crate::W<u32, super::PMD>;
#[doc = "Register PMD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PMD0`"]
pub type PMD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD0`"]
pub struct PMD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PMD1`"]
pub type PMD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD1`"]
pub struct PMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PMD2`"]
pub type PMD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD2`"]
pub struct PMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PMD3`"]
pub type PMD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD3`"]
pub struct PMD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PMD4`"]
pub type PMD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD4`"]
pub struct PMD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PMD5`"]
pub type PMD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD5`"]
pub struct PMD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PMD6`"]
pub type PMD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD6`"]
pub struct PMD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PMD7`"]
pub type PMD7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD7`"]
pub struct PMD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PMD8`"]
pub type PMD8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD8`"]
pub struct PMD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PMD9`"]
pub type PMD9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD9`"]
pub struct PMD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PMD10`"]
pub type PMD10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD10`"]
pub struct PMD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PMD11`"]
pub type PMD11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD11`"]
pub struct PMD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PMD12`"]
pub type PMD12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD12`"]
pub struct PMD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PMD13`"]
pub type PMD13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD13`"]
pub struct PMD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PMD14`"]
pub type PMD14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD14`"]
pub struct PMD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `PMD15`"]
pub type PMD15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMD15`"]
pub struct PMD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PMD15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd0(&self) -> PMD0_R {
        PMD0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd1(&self) -> PMD1_R {
        PMD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd2(&self) -> PMD2_R {
        PMD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd3(&self) -> PMD3_R {
        PMD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd4(&self) -> PMD4_R {
        PMD4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd5(&self) -> PMD5_R {
        PMD5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd6(&self) -> PMD6_R {
        PMD6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd7(&self) -> PMD7_R {
        PMD7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd8(&self) -> PMD8_R {
        PMD8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd9(&self) -> PMD9_R {
        PMD9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd10(&self) -> PMD10_R {
        PMD10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd11(&self) -> PMD11_R {
        PMD11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd12(&self) -> PMD12_R {
        PMD12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd13(&self) -> PMD13_R {
        PMD13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd14(&self) -> PMD14_R {
        PMD14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd15(&self) -> PMD15_R {
        PMD15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd0(&mut self) -> PMD0_W {
        PMD0_W { w: self }
    }
    #[doc = "Bits 2:3 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd1(&mut self) -> PMD1_W {
        PMD1_W { w: self }
    }
    #[doc = "Bits 4:5 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd2(&mut self) -> PMD2_W {
        PMD2_W { w: self }
    }
    #[doc = "Bits 6:7 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd3(&mut self) -> PMD3_W {
        PMD3_W { w: self }
    }
    #[doc = "Bits 8:9 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd4(&mut self) -> PMD4_W {
        PMD4_W { w: self }
    }
    #[doc = "Bits 10:11 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd5(&mut self) -> PMD5_W {
        PMD5_W { w: self }
    }
    #[doc = "Bits 12:13 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd6(&mut self) -> PMD6_W {
        PMD6_W { w: self }
    }
    #[doc = "Bits 14:15 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd7(&mut self) -> PMD7_W {
        PMD7_W { w: self }
    }
    #[doc = "Bits 16:17 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd8(&mut self) -> PMD8_W {
        PMD8_W { w: self }
    }
    #[doc = "Bits 18:19 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd9(&mut self) -> PMD9_W {
        PMD9_W { w: self }
    }
    #[doc = "Bits 20:21 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd10(&mut self) -> PMD10_W {
        PMD10_W { w: self }
    }
    #[doc = "Bits 22:23 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd11(&mut self) -> PMD11_W {
        PMD11_W { w: self }
    }
    #[doc = "Bits 24:25 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd12(&mut self) -> PMD12_W {
        PMD12_W { w: self }
    }
    #[doc = "Bits 26:27 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd13(&mut self) -> PMD13_W {
        PMD13_W { w: self }
    }
    #[doc = "Bits 28:29 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd14(&mut self) -> PMD14_W {
        PMD14_W { w: self }
    }
    #[doc = "Bits 30:31 - GPIOX I/O Pin\\[n\\]
Mode Control Determine each I/O type of GPIOx pins 00 = GPIO port \\[n\\]
pin is in INPUT mode. 01 = GPIO port \\[n\\]
pin is in OUTPUT mode. 10 = GPIO port \\[n\\]
pin is in Open-Drain mode. 11 = GPIO port \\[n\\]
pin is in Quasi-bidirectional mode."]
    #[inline(always)]
    pub fn pmd15(&mut self) -> PMD15_W {
        PMD15_W { w: self }
    }
}
