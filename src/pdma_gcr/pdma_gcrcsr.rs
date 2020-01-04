#[doc = "Reader of register PDMA_GCRCSR"]
pub type R = crate::R<u32, super::PDMA_GCRCSR>;
#[doc = "Writer for register PDMA_GCRCSR"]
pub type W = crate::W<u32, super::PDMA_GCRCSR>;
#[doc = "Register PDMA_GCRCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_GCRCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK0_EN`"]
pub type CLK0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK0_EN`"]
pub struct CLK0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK0_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLK1_EN`"]
pub type CLK1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK1_EN`"]
pub struct CLK1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CLK2_EN`"]
pub type CLK2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK2_EN`"]
pub struct CLK2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK2_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLK3_EN`"]
pub type CLK3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK3_EN`"]
pub struct CLK3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK3_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CLK4_EN`"]
pub type CLK4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK4_EN`"]
pub struct CLK4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK4_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CLK5_EN`"]
pub type CLK5_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK5_EN`"]
pub struct CLK5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK5_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CLK6_EN`"]
pub type CLK6_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK6_EN`"]
pub struct CLK6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK6_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CLK7_EN`"]
pub type CLK7_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK7_EN`"]
pub struct CLK7_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK7_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CLK8_EN`"]
pub type CLK8_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK8_EN`"]
pub struct CLK8_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK8_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - PDMA Controller Channel 0 Clock Enable Control 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk0_en(&self) -> CLK0_EN_R {
        CLK0_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PDMA Controller Channel 1 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk1_en(&self) -> CLK1_EN_R {
        CLK1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PDMA Controller Channel 2 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk2_en(&self) -> CLK2_EN_R {
        CLK2_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PDMA Controller Channel 3 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk3_en(&self) -> CLK3_EN_R {
        CLK3_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PDMA Controller Channel 4 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk4_en(&self) -> CLK4_EN_R {
        CLK4_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDMA Controller Channel 5 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk5_en(&self) -> CLK5_EN_R {
        CLK5_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDMA Controller Channel 6 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk6_en(&self) -> CLK6_EN_R {
        CLK6_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PDMA Controller Channel 7 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk7_en(&self) -> CLK7_EN_R {
        CLK7_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PDMA Controller Channel 8 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk8_en(&self) -> CLK8_EN_R {
        CLK8_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PDMA Controller Channel 0 Clock Enable Control 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk0_en(&mut self) -> CLK0_EN_W {
        CLK0_EN_W { w: self }
    }
    #[doc = "Bit 9 - PDMA Controller Channel 1 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk1_en(&mut self) -> CLK1_EN_W {
        CLK1_EN_W { w: self }
    }
    #[doc = "Bit 10 - PDMA Controller Channel 2 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk2_en(&mut self) -> CLK2_EN_W {
        CLK2_EN_W { w: self }
    }
    #[doc = "Bit 11 - PDMA Controller Channel 3 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk3_en(&mut self) -> CLK3_EN_W {
        CLK3_EN_W { w: self }
    }
    #[doc = "Bit 12 - PDMA Controller Channel 4 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk4_en(&mut self) -> CLK4_EN_W {
        CLK4_EN_W { w: self }
    }
    #[doc = "Bit 13 - PDMA Controller Channel 5 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk5_en(&mut self) -> CLK5_EN_W {
        CLK5_EN_W { w: self }
    }
    #[doc = "Bit 14 - PDMA Controller Channel 6 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk6_en(&mut self) -> CLK6_EN_W {
        CLK6_EN_W { w: self }
    }
    #[doc = "Bit 15 - PDMA Controller Channel 7 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk7_en(&mut self) -> CLK7_EN_W {
        CLK7_EN_W { w: self }
    }
    #[doc = "Bit 16 - PDMA Controller Channel 8 Clock Enable Control(Medium Density Only) 0 = Disable 1 = Enable"]
    #[inline(always)]
    pub fn clk8_en(&mut self) -> CLK8_EN_W {
        CLK8_EN_W { w: self }
    }
}
