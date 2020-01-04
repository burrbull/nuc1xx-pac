#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IF_EN0`"]
pub type IF_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN0`"]
pub struct IF_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN0_W<'a> {
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
#[doc = "Reader of field `IF_EN1`"]
pub type IF_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN1`"]
pub struct IF_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN1_W<'a> {
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
#[doc = "Reader of field `IF_EN2`"]
pub type IF_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN2`"]
pub struct IF_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN2_W<'a> {
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
#[doc = "Reader of field `IF_EN3`"]
pub type IF_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN3`"]
pub struct IF_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `IF_EN4`"]
pub type IF_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN4`"]
pub struct IF_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN4_W<'a> {
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
#[doc = "Reader of field `IF_EN5`"]
pub type IF_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN5`"]
pub struct IF_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN5_W<'a> {
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
#[doc = "Reader of field `IF_EN6`"]
pub type IF_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN6`"]
pub struct IF_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IF_EN7`"]
pub type IF_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN7`"]
pub struct IF_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IF_EN8`"]
pub type IF_EN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN8`"]
pub struct IF_EN8_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN8_W<'a> {
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
#[doc = "Reader of field `IF_EN9`"]
pub type IF_EN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN9`"]
pub struct IF_EN9_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN9_W<'a> {
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
#[doc = "Reader of field `IF_EN10`"]
pub type IF_EN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN10`"]
pub struct IF_EN10_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN10_W<'a> {
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
#[doc = "Reader of field `IF_EN11`"]
pub type IF_EN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN11`"]
pub struct IF_EN11_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN11_W<'a> {
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
#[doc = "Reader of field `IF_EN12`"]
pub type IF_EN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN12`"]
pub struct IF_EN12_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN12_W<'a> {
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
#[doc = "Reader of field `IF_EN13`"]
pub type IF_EN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN13`"]
pub struct IF_EN13_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN13_W<'a> {
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
#[doc = "Reader of field `IF_EN14`"]
pub type IF_EN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN14`"]
pub struct IF_EN14_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN14_W<'a> {
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
#[doc = "Reader of field `IF_EN15`"]
pub type IF_EN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_EN15`"]
pub struct IF_EN15_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_EN15_W<'a> {
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
#[doc = "Reader of field `IR_EN0`"]
pub type IR_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN0`"]
pub struct IR_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN0_W<'a> {
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
#[doc = "Reader of field `IR_EN1`"]
pub type IR_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN1`"]
pub struct IR_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `IR_EN2`"]
pub type IR_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN2`"]
pub struct IR_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `IR_EN3`"]
pub type IR_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN3`"]
pub struct IR_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `IR_EN4`"]
pub type IR_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN4`"]
pub struct IR_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `IR_EN5`"]
pub type IR_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN5`"]
pub struct IR_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `IR_EN6`"]
pub type IR_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN6`"]
pub struct IR_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `IR_EN7`"]
pub type IR_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN7`"]
pub struct IR_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `IR_EN8`"]
pub type IR_EN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN8`"]
pub struct IR_EN8_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `IR_EN9`"]
pub type IR_EN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN9`"]
pub struct IR_EN9_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `IR_EN10`"]
pub type IR_EN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN10`"]
pub struct IR_EN10_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `IR_EN11`"]
pub type IR_EN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN11`"]
pub struct IR_EN11_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `IR_EN12`"]
pub type IR_EN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN12`"]
pub struct IR_EN12_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `IR_EN13`"]
pub type IR_EN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN13`"]
pub struct IR_EN13_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IR_EN14`"]
pub type IR_EN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN14`"]
pub struct IR_EN14_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `IR_EN15`"]
pub type IR_EN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IR_EN15`"]
pub struct IR_EN15_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_EN15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en0(&self) -> IF_EN0_R {
        IF_EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en1(&self) -> IF_EN1_R {
        IF_EN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en2(&self) -> IF_EN2_R {
        IF_EN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en3(&self) -> IF_EN3_R {
        IF_EN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en4(&self) -> IF_EN4_R {
        IF_EN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en5(&self) -> IF_EN5_R {
        IF_EN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en6(&self) -> IF_EN6_R {
        IF_EN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en7(&self) -> IF_EN7_R {
        IF_EN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en8(&self) -> IF_EN8_R {
        IF_EN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en9(&self) -> IF_EN9_R {
        IF_EN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en10(&self) -> IF_EN10_R {
        IF_EN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en11(&self) -> IF_EN11_R {
        IF_EN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en12(&self) -> IF_EN12_R {
        IF_EN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en13(&self) -> IF_EN13_R {
        IF_EN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en14(&self) -> IF_EN14_R {
        IF_EN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en15(&self) -> IF_EN15_R {
        IF_EN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en0(&self) -> IR_EN0_R {
        IR_EN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en1(&self) -> IR_EN1_R {
        IR_EN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en2(&self) -> IR_EN2_R {
        IR_EN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en3(&self) -> IR_EN3_R {
        IR_EN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en4(&self) -> IR_EN4_R {
        IR_EN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en5(&self) -> IR_EN5_R {
        IR_EN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en6(&self) -> IR_EN6_R {
        IR_EN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en7(&self) -> IR_EN7_R {
        IR_EN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en8(&self) -> IR_EN8_R {
        IR_EN8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en9(&self) -> IR_EN9_R {
        IR_EN9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en10(&self) -> IR_EN10_R {
        IR_EN10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en11(&self) -> IR_EN11_R {
        IR_EN11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en12(&self) -> IR_EN12_R {
        IR_EN12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en13(&self) -> IR_EN13_R {
        IR_EN13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en14(&self) -> IR_EN14_R {
        IR_EN14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en15(&self) -> IR_EN15_R {
        IR_EN15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en0(&mut self) -> IF_EN0_W {
        IF_EN0_W { w: self }
    }
    #[doc = "Bit 1 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en1(&mut self) -> IF_EN1_W {
        IF_EN1_W { w: self }
    }
    #[doc = "Bit 2 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en2(&mut self) -> IF_EN2_W {
        IF_EN2_W { w: self }
    }
    #[doc = "Bit 3 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en3(&mut self) -> IF_EN3_W {
        IF_EN3_W { w: self }
    }
    #[doc = "Bit 4 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en4(&mut self) -> IF_EN4_W {
        IF_EN4_W { w: self }
    }
    #[doc = "Bit 5 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en5(&mut self) -> IF_EN5_W {
        IF_EN5_W { w: self }
    }
    #[doc = "Bit 6 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en6(&mut self) -> IF_EN6_W {
        IF_EN6_W { w: self }
    }
    #[doc = "Bit 7 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en7(&mut self) -> IF_EN7_W {
        IF_EN7_W { w: self }
    }
    #[doc = "Bit 8 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en8(&mut self) -> IF_EN8_W {
        IF_EN8_W { w: self }
    }
    #[doc = "Bit 9 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en9(&mut self) -> IF_EN9_W {
        IF_EN9_W { w: self }
    }
    #[doc = "Bit 10 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en10(&mut self) -> IF_EN10_W {
        IF_EN10_W { w: self }
    }
    #[doc = "Bit 11 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en11(&mut self) -> IF_EN11_W {
        IF_EN11_W { w: self }
    }
    #[doc = "Bit 12 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en12(&mut self) -> IF_EN12_W {
        IF_EN12_W { w: self }
    }
    #[doc = "Bit 13 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en13(&mut self) -> IF_EN13_W {
        IF_EN13_W { w: self }
    }
    #[doc = "Bit 14 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en14(&mut self) -> IF_EN14_W {
        IF_EN14_W { w: self }
    }
    #[doc = "Bit 15 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Falling Edge or Input Level Low IF_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IF_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"low\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"high-to-low\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
state low-level or high-to-low change interrupt 0 = Disable the PIN\\[n\\]
state low-level or high-to-low change interrupt"]
    #[inline(always)]
    pub fn if_en15(&mut self) -> IF_EN15_W {
        IF_EN15_W { w: self }
    }
    #[doc = "Bit 16 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en0(&mut self) -> IR_EN0_W {
        IR_EN0_W { w: self }
    }
    #[doc = "Bit 17 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en1(&mut self) -> IR_EN1_W {
        IR_EN1_W { w: self }
    }
    #[doc = "Bit 18 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en2(&mut self) -> IR_EN2_W {
        IR_EN2_W { w: self }
    }
    #[doc = "Bit 19 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en3(&mut self) -> IR_EN3_W {
        IR_EN3_W { w: self }
    }
    #[doc = "Bit 20 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en4(&mut self) -> IR_EN4_W {
        IR_EN4_W { w: self }
    }
    #[doc = "Bit 21 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en5(&mut self) -> IR_EN5_W {
        IR_EN5_W { w: self }
    }
    #[doc = "Bit 22 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en6(&mut self) -> IR_EN6_W {
        IR_EN6_W { w: self }
    }
    #[doc = "Bit 23 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en7(&mut self) -> IR_EN7_W {
        IR_EN7_W { w: self }
    }
    #[doc = "Bit 24 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en8(&mut self) -> IR_EN8_W {
        IR_EN8_W { w: self }
    }
    #[doc = "Bit 25 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en9(&mut self) -> IR_EN9_W {
        IR_EN9_W { w: self }
    }
    #[doc = "Bit 26 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en10(&mut self) -> IR_EN10_W {
        IR_EN10_W { w: self }
    }
    #[doc = "Bit 27 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en11(&mut self) -> IR_EN11_W {
        IR_EN11_W { w: self }
    }
    #[doc = "Bit 28 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en12(&mut self) -> IR_EN12_W {
        IR_EN12_W { w: self }
    }
    #[doc = "Bit 29 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en13(&mut self) -> IR_EN13_W {
        IR_EN13_W { w: self }
    }
    #[doc = "Bit 30 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en14(&mut self) -> IR_EN14_W {
        IR_EN14_W { w: self }
    }
    #[doc = "Bit 31 - Port \\[A/B/C/D/E\\]
Interrupt Enable by Input Rising Edge or Input Level High IR_EN\\[n\\]
used to enable the interrupt for each of the corresponding input GPIO_PIN\\[n\\]. Set bit to 1 also enable the pin wakeup function When set the IR_EN\\[n\\]
bit to 1: If the interrupt is level trigger, the input PIN\\[n\\]
state at level \"high\" will generate the interrupt. If the interrupt is edge trigger, the input PIN\\[n\\]
state change from \"low-to-high\" will generate the interrupt. 1 = Enable the PIN\\[n\\]
level-high or low-to-high interrupt 0 = Disable the PIN\\[n\\]
level-high or low-to-high interrupt."]
    #[inline(always)]
    pub fn ir_en15(&mut self) -> IR_EN15_W {
        IR_EN15_W { w: self }
    }
}
