#[doc = "Reader of register CMP1CR"]
pub type R = crate::R<u32, super::CMP1CR>;
#[doc = "Writer for register CMP1CR"]
pub type W = crate::W<u32, super::CMP1CR>;
#[doc = "Register CMP1CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP1CR {
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
#[doc = "Reader of field `CN1`"]
pub type CN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CN1`"]
pub struct CN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CN1_W<'a> {
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
    #[doc = "Bit 0 - Comparator1 Enable 1 = Enable 0 = Disable Comparator1 output needs wait 10 us stable time after CMP1EN is set"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator1 Interrupt Enable 1 = Enable Comparator1 interrupt function 0 = Disable Comparator1 interrupt function Interrupt is generated if CMP1IE bit is set to 1 after comparator1 conversion finished."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator1 Hysterisis Enable 1 = Enable comparator1 Hysterisis function; the typical range is 20mV. 0 = Disable comparator1 Hysterisis function (Default)."]
    #[inline(always)]
    pub fn hysen(&self) -> HYSEN_R {
        HYSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator1 negative input select 1 = The internal comparator reference voltage (Vref=1.2V) is selected as the negative comparator input 0 = The comparator1 reference pin CPN1 is selected as the negative comparator input"]
    #[inline(always)]
    pub fn cn1(&self) -> CN1_R {
        CN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator1 Enable 1 = Enable 0 = Disable Comparator1 output needs wait 10 us stable time after CMP1EN is set"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Comparator1 Interrupt Enable 1 = Enable Comparator1 interrupt function 0 = Disable Comparator1 interrupt function Interrupt is generated if CMP1IE bit is set to 1 after comparator1 conversion finished."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Comparator1 Hysterisis Enable 1 = Enable comparator1 Hysterisis function; the typical range is 20mV. 0 = Disable comparator1 Hysterisis function (Default)."]
    #[inline(always)]
    pub fn hysen(&mut self) -> HYSEN_W {
        HYSEN_W { w: self }
    }
    #[doc = "Bit 4 - Comparator1 negative input select 1 = The internal comparator reference voltage (Vref=1.2V) is selected as the negative comparator input 0 = The comparator1 reference pin CPN1 is selected as the negative comparator input"]
    #[inline(always)]
    pub fn cn1(&mut self) -> CN1_W {
        CN1_W { w: self }
    }
}
