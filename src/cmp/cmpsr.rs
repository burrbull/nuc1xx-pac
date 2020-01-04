#[doc = "Reader of register CMPSR"]
pub type R = crate::R<u32, super::CMPSR>;
#[doc = "Writer for register CMPSR"]
pub type W = crate::W<u32, super::CMPSR>;
#[doc = "Register CMPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPF0`"]
pub type CMPF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPF0`"]
pub struct CMPF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF0_W<'a> {
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
#[doc = "Reader of field `CMPF1`"]
pub type CMPF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPF1`"]
pub struct CMPF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF1_W<'a> {
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
#[doc = "Reader of field `CO0`"]
pub type CO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CO1`"]
pub type CO1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparator0 Flag This bit is set by hardware whenever the comparator0 output changes state. This will cause an interrupt if CMP0IE set. Write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn cmpf0(&self) -> CMPF0_R {
        CMPF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator1 Flag This bit is set by hardware whenever the comparator1 output changes state. This will cause an interrupt if CMP1IE set. Write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn cmpf1(&self) -> CMPF1_R {
        CMPF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator0 Output Synchronized to the APB clock to allow reading by software. Cleared when the comparator is disabled (CMP0EN = 0)."]
    #[inline(always)]
    pub fn co0(&self) -> CO0_R {
        CO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator1 Output Synchronized to the APB clock to allow reading by software. Cleared when the comparator is disabled (CMP1EN = 0)."]
    #[inline(always)]
    pub fn co1(&self) -> CO1_R {
        CO1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator0 Flag This bit is set by hardware whenever the comparator0 output changes state. This will cause an interrupt if CMP0IE set. Write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn cmpf0(&mut self) -> CMPF0_W {
        CMPF0_W { w: self }
    }
    #[doc = "Bit 1 - Comparator1 Flag This bit is set by hardware whenever the comparator1 output changes state. This will cause an interrupt if CMP1IE set. Write 1 to clear this bit to zero."]
    #[inline(always)]
    pub fn cmpf1(&mut self) -> CMPF1_W {
        CMPF1_W { w: self }
    }
}
