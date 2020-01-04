#[doc = "Reader of register UA_MCR"]
pub type R = crate::R<u32, super::UA_MCR>;
#[doc = "Writer for register UA_MCR"]
pub type W = crate::W<u32, super::UA_MCR>;
#[doc = "Register UA_MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTS`"]
pub type RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS`"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
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
#[doc = "Reader of field `LEV_RTS`"]
pub type LEV_RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV_RTS`"]
pub struct LEV_RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV_RTS_W<'a> {
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
#[doc = "Reader of field `RTS_ST`"]
pub type RTS_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - RTS (Request-To-Send) Signal (not available in UART2 channel) 0: Drive RTS pin to logic 1 (If the LEV_RTS set to low level triggered). 1: Drive RTS pin to logic 0 (If the LEV_RTS set to low level triggered). 0: Drive RTS pin to logic 0 (If the LEV_RTS set to hihg level triggered). 1: Drive RTS pin to logic 1 (If the LEV_RTS set to high level triggered)."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTS Trigger Level (not available in UART2 channel) This bit can change the RTS trigger level. 0= low level triggered 1= high level triggered"]
    #[inline(always)]
    pub fn lev_rts(&self) -> LEV_RTS_R {
        LEV_RTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RTS Pin State (not available in UART2 channel) This bit is the output pin status of RTS."]
    #[inline(always)]
    pub fn rts_st(&self) -> RTS_ST_R {
        RTS_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RTS (Request-To-Send) Signal (not available in UART2 channel) 0: Drive RTS pin to logic 1 (If the LEV_RTS set to low level triggered). 1: Drive RTS pin to logic 0 (If the LEV_RTS set to low level triggered). 0: Drive RTS pin to logic 0 (If the LEV_RTS set to hihg level triggered). 1: Drive RTS pin to logic 1 (If the LEV_RTS set to high level triggered)."]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 9 - RTS Trigger Level (not available in UART2 channel) This bit can change the RTS trigger level. 0= low level triggered 1= high level triggered"]
    #[inline(always)]
    pub fn lev_rts(&mut self) -> LEV_RTS_W {
        LEV_RTS_W { w: self }
    }
}
