#[doc = "Reader of register GPE_MFP"]
pub type R = crate::R<u32, super::GPE_MFP>;
#[doc = "Writer for register GPE_MFP"]
pub type W = crate::W<u32, super::GPE_MFP>;
#[doc = "Register GPE_MFP `reset()`'s with value 0"]
impl crate::ResetValue for super::GPE_MFP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPE_MFP0`"]
pub type GPE_MFP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPE_MFP0`"]
pub struct GPE_MFP0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPE_MFP0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `GPE_MFP1`"]
pub type GPE_MFP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPE_MFP1`"]
pub struct GPE_MFP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPE_MFP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `GPE_MFP5`"]
pub type GPE_MFP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPE_MFP5`"]
pub struct GPE_MFP5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPE_MFP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GPE_TYPEn`"]
pub type GPE_TYPEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPE_TYPEn`"]
pub struct GPE_TYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPE_TYPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PE.0 Pin Function Selection (Medium Density Only) 1 = The PWM6 function is selected to the pin PE.0 0 = The GPIOE\\[0\\]
is selected to the pin PE.0"]
    #[inline(always)]
    pub fn gpe_mfp0(&self) -> GPE_MFP0_R {
        GPE_MFP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PE.1 Pin Function Selection (Medium Density Only) 1 = The PWM7 function is selected to the pin PE.1 0 = The GPIOE\\[1\\]
is selected to the pin PE.1"]
    #[inline(always)]
    pub fn gpe_mfp1(&self) -> GPE_MFP1_R {
        GPE_MFP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PE.5 Pin Function Selection (Medium Density Only) 1 = The PWM5 function is selected to the pin PE.5 0 = The GPIOE\\[5\\]
is selected to the pin PE.5"]
    #[inline(always)]
    pub fn gpe_mfp5(&self) -> GPE_MFP5_R {
        GPE_MFP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOE\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOE\\[15:0\\]
I/O input Schmitt Trigger function Note: In this field, Low Density only has GPE_TYPE5 bit"]
    #[inline(always)]
    pub fn gpe_typen(&self) -> GPE_TYPEN_R {
        GPE_TYPEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PE.0 Pin Function Selection (Medium Density Only) 1 = The PWM6 function is selected to the pin PE.0 0 = The GPIOE\\[0\\]
is selected to the pin PE.0"]
    #[inline(always)]
    pub fn gpe_mfp0(&mut self) -> GPE_MFP0_W {
        GPE_MFP0_W { w: self }
    }
    #[doc = "Bit 1 - PE.1 Pin Function Selection (Medium Density Only) 1 = The PWM7 function is selected to the pin PE.1 0 = The GPIOE\\[1\\]
is selected to the pin PE.1"]
    #[inline(always)]
    pub fn gpe_mfp1(&mut self) -> GPE_MFP1_W {
        GPE_MFP1_W { w: self }
    }
    #[doc = "Bit 5 - PE.5 Pin Function Selection (Medium Density Only) 1 = The PWM5 function is selected to the pin PE.5 0 = The GPIOE\\[5\\]
is selected to the pin PE.5"]
    #[inline(always)]
    pub fn gpe_mfp5(&mut self) -> GPE_MFP5_W {
        GPE_MFP5_W { w: self }
    }
    #[doc = "Bits 16:31 - 1 = Enable GPIOE\\[15:0\\]
I/O input Schmitt Trigger function 0 = Disable GPIOE\\[15:0\\]
I/O input Schmitt Trigger function Note: In this field, Low Density only has GPE_TYPE5 bit"]
    #[inline(always)]
    pub fn gpe_typen(&mut self) -> GPE_TYPEN_W {
        GPE_TYPEN_W { w: self }
    }
}
