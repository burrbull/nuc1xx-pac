#[doc = "Reader of register TSSR"]
pub type R = crate::R<u32, super::TSSR>;
#[doc = "Writer for register TSSR"]
pub type W = crate::W<u32, super::TSSR>;
#[doc = "Register TSSR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TSSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `_24H_12H`"]
pub type _24H_12H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `_24H_12H`"]
pub struct _24H_12H_W<'a> {
    w: &'a mut W,
}
impl<'a> _24H_12H_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 24-Hour / 12-Hour Time Scale Selection It indicate that TLR and TAR are in 24-hour time mode or 12-hour time mode 1 = select 24-hour time scale 0 = select 12-hour time scale with AM and PM indication 24-hour time scale 12-hour time scale 24-hour time scale 12-hour time scale (PM time + 20) 00 12(AM12) 12 32(PM12) 01 01(AM01) 13 21(PM01) 02 02(AM02) 14 22(PM02) 03 03(AM03) 15 23(PM03) 04 04(AM04) 16 24(PM04) 05 05(AM05) 17 25(PM05) 06 06(AM06) 18 26(PM06) 07 07(AM07) 19 27(PM07) 08 08(AM08) 20 28(PM08) 09 09(AM09) 21 29(PM09) 10 10(AM10) 22 30(PM10) 11 11(AM11) 23 31(PM11)"]
    #[inline(always)]
    pub fn _24h_12h(&self) -> _24H_12H_R {
        _24H_12H_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 24-Hour / 12-Hour Time Scale Selection It indicate that TLR and TAR are in 24-hour time mode or 12-hour time mode 1 = select 24-hour time scale 0 = select 12-hour time scale with AM and PM indication 24-hour time scale 12-hour time scale 24-hour time scale 12-hour time scale (PM time + 20) 00 12(AM12) 12 32(PM12) 01 01(AM01) 13 21(PM01) 02 02(AM02) 14 22(PM02) 03 03(AM03) 15 23(PM03) 04 04(AM04) 16 24(PM04) 05 05(AM05) 17 25(PM05) 06 06(AM06) 18 26(PM06) 07 07(AM07) 19 27(PM07) 08 08(AM08) 20 28(PM08) 09 09(AM09) 21 29(PM09) 10 10(AM10) 22 30(PM10) 11 11(AM11) 23 31(PM11)"]
    #[inline(always)]
    pub fn _24h_12h(&mut self) -> _24H_12H_W {
        _24H_12H_W { w: self }
    }
}
