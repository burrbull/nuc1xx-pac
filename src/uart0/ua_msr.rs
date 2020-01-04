#[doc = "Reader of register UA_MSR"]
pub type R = crate::R<u32, super::UA_MSR>;
#[doc = "Writer for register UA_MSR"]
pub type W = crate::W<u32, super::UA_MSR>;
#[doc = "Register UA_MSR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_MSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCTSF`"]
pub type DCTSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTS_ST`"]
pub type CTS_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEV_CTS`"]
pub type LEV_CTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEV_CTS`"]
pub struct LEV_CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEV_CTS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Detect CTS State Change Flag (not available in UART2 channel) This bit is set whenever CTS input has change state, and it will generate Modem interrupt to CPU when UA_IER \\[Modem_IEN\\]
is set to 1. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn dctsf(&self) -> DCTSF_R {
        DCTSF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTS Pin Status (not available in UART2 channel) This bit is the pin status of CTS."]
    #[inline(always)]
    pub fn cts_st(&self) -> CTS_ST_R {
        CTS_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CTS Trigger Level (not available in UART2 channel) This bit can change the CTS trigger level. 0= low level triggered 1= high level triggered"]
    #[inline(always)]
    pub fn lev_cts(&self) -> LEV_CTS_R {
        LEV_CTS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CTS Trigger Level (not available in UART2 channel) This bit can change the CTS trigger level. 0= low level triggered 1= high level triggered"]
    #[inline(always)]
    pub fn lev_cts(&mut self) -> LEV_CTS_W {
        LEV_CTS_W { w: self }
    }
}
