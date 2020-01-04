#[doc = "Reader of register CMP0CR"]
pub type R = crate::R<u32, super::CMP0CR>;
#[doc = "Writer for register CMP0CR"]
pub type W = crate::W<u32, super::CMP0CR>;
#[doc = "Register CMP0CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP0CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Reader of field `HYSEN`"]
pub type HYSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYSEN`"]
pub struct HYSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CN0`"]
pub type CN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CN0`"]
pub struct CN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator0 Enable 1 = Enable 0 = Disable Comparator0 output needs wait 10 us stable time after CMP0EN is set"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator0 Interrupt Enable 1 = Enable comparator0 interrupt function 0 = Disable comparator0 interrupt function Interrupt is generated if CMP0IE bit is set to 1 after comparator0 conversion finished."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CMP0 Hysterisis Enable 1 = Enable comparator0 Hysterisis function; the typical range is 20mV. 0 = Disable comparator0 Hysterisis function (Default)."]
    #[inline(always)]
    pub fn hysen(&self) -> HYSEN_R {
        HYSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator0 negative input select 1 = The internal comparator reference voltage (Vref=1.2V) is selected as the negative comparator input 0 = The comparator0 reference pin CPN0 is selected as the negative comparator input"]
    #[inline(always)]
    pub fn cn0(&self) -> CN0_R {
        CN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator0 Enable 1 = Enable 0 = Disable Comparator0 output needs wait 10 us stable time after CMP0EN is set"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Comparator0 Interrupt Enable 1 = Enable comparator0 interrupt function 0 = Disable comparator0 interrupt function Interrupt is generated if CMP0IE bit is set to 1 after comparator0 conversion finished."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - CMP0 Hysterisis Enable 1 = Enable comparator0 Hysterisis function; the typical range is 20mV. 0 = Disable comparator0 Hysterisis function (Default)."]
    #[inline(always)]
    pub fn hysen(&mut self) -> HYSEN_W {
        HYSEN_W { w: self }
    }
    #[doc = "Bit 4 - Comparator0 negative input select 1 = The internal comparator reference voltage (Vref=1.2V) is selected as the negative comparator input 0 = The comparator0 reference pin CPN0 is selected as the negative comparator input"]
    #[inline(always)]
    pub fn cn0(&mut self) -> CN0_W {
        CN0_W { w: self }
    }
}
