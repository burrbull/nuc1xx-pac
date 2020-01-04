#[doc = "Reader of register FCR"]
pub type R = crate::R<u32, super::FCR>;
#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0x0700"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700
    }
}
#[doc = "Reader of field `FRACTION`"]
pub type FRACTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACTION`"]
pub struct FRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `INTEGER`"]
pub type INTEGER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTEGER`"]
pub struct INTEGER_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fraction Part Formula = (fraction part of detected value) x 60 Note: Digit in FCR must be expressed as hexadecimal number. Refer to 5.8.4.4 for the examples."]
    #[inline(always)]
    pub fn fraction(&self) -> FRACTION_R {
        FRACTION_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Integer Part Integer part of detected value FCR\\[11:8\\]
Integer part of detected value FCR\\[11:8\\]
32776 1111 32768 0111 32775 1110 32767 0110 32774 1101 32766 0101 32773 1100 32765 0100 32772 1011 32764 0011 32771 1010 32763 0010 32770 1001 32762 0001 32769 1000 32761 0000"]
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fraction Part Formula = (fraction part of detected value) x 60 Note: Digit in FCR must be expressed as hexadecimal number. Refer to 5.8.4.4 for the examples."]
    #[inline(always)]
    pub fn fraction(&mut self) -> FRACTION_W {
        FRACTION_W { w: self }
    }
    #[doc = "Bits 8:11 - Integer Part Integer part of detected value FCR\\[11:8\\]
Integer part of detected value FCR\\[11:8\\]
32776 1111 32768 0111 32775 1110 32767 0110 32774 1101 32766 0101 32773 1100 32765 0100 32772 1011 32764 0011 32771 1010 32763 0010 32770 1001 32762 0001 32769 1000 32761 0000"]
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W {
        INTEGER_W { w: self }
    }
}
