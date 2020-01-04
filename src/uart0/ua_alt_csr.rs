#[doc = "Reader of register UA_ALT_CSR"]
pub type R = crate::R<u32, super::UA_ALT_CSR>;
#[doc = "Writer for register UA_ALT_CSR"]
pub type W = crate::W<u32, super::UA_ALT_CSR>;
#[doc = "Register UA_ALT_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_ALT_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UA_LIN_BKFL`"]
pub type UA_LIN_BKFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UA_LIN_BKFL`"]
pub struct UA_LIN_BKFL_W<'a> {
    w: &'a mut W,
}
impl<'a> UA_LIN_BKFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LIN_RX_EN`"]
pub type LIN_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIN_RX_EN`"]
pub struct LIN_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_RX_EN_W<'a> {
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
#[doc = "Reader of field `LIN_TX_EN`"]
pub type LIN_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIN_TX_EN`"]
pub struct LIN_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_TX_EN_W<'a> {
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
#[doc = "Reader of field `RS485_NMM`"]
pub type RS485_NMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_NMM`"]
pub struct RS485_NMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_NMM_W<'a> {
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
#[doc = "Reader of field `RS485_AAD`"]
pub type RS485_AAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_AAD`"]
pub struct RS485_AAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_AAD_W<'a> {
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
#[doc = "Reader of field `RS485_AUD`"]
pub type RS485_AUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_AUD`"]
pub struct RS485_AUD_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_AUD_W<'a> {
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
#[doc = "Reader of field `RS485_ADD_EN`"]
pub type RS485_ADD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS485_ADD_EN`"]
pub struct RS485_ADD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_ADD_EN_W<'a> {
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
#[doc = "Reader of field `ADDR_MATCH`"]
pub type ADDR_MATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR_MATCH`"]
pub struct ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - UART LIN Break Field Length This field indicates a 4-bit LIN TX break field count. NOTE: This break field length is UA_LIN_BKFL + 2"]
    #[inline(always)]
    pub fn ua_lin_bkfl(&self) -> UA_LIN_BKFL_R {
        UA_LIN_BKFL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - LIN RX Enable 1 = Enable LIN RX mode. 0 = Disable LIN RX mode."]
    #[inline(always)]
    pub fn lin_rx_en(&self) -> LIN_RX_EN_R {
        LIN_RX_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN TX Break Mode Enable 1 = Enable LIN TX Break mode. 0 = Disable LIN TX Break mode. NOTE: When TX break field transfer operation finish, this will be cleared automatically."]
    #[inline(always)]
    pub fn lin_tx_en(&self) -> LIN_TX_EN_R {
        LIN_TX_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RS-485 Normal Multi-drop Operation Mode (NMM) (Low Density Only) 1 = Enable RS-485 Normal Multi-drop Operation Mode (NMM). 0 = Disable RS-485 Normal Multi-drop Operation Mode (NMM). Note: It can't be active with RS485_AAD operation mode."]
    #[inline(always)]
    pub fn rs485_nmm(&self) -> RS485_NMM_R {
        RS485_NMM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RS-485 Auto Address Detection Operation Mode (AAD) (Low Density Only) 1 = Enable RS-485 Auto Address Detection Operation Mode (AAD). 0 = Disable RS-485 Auto Address Detection Operation Mode (AAD). Note: It can't be active with RS485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485_aad(&self) -> RS485_AAD_R {
        RS485_AAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RS-485 Auto Direction Mode (AUD) (Low Density Only) 1 = Enable RS-485 Auto Direction Operation Mode (AUO). 0 = Disable RS-485 Auto Direction Operation Mode (AUO). Note:This field is used for RS-485 any operation mode. Note: It can be active with RS-485_AAD or RS485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485_aud(&self) -> RS485_AUD_R {
        RS485_AUD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RS-485 Address Detection Enable (Low Density Only) 1 = Enable address detection mode. 0 = Disable address detection mode. Note: This field is used for RS485 any operation mode."]
    #[inline(always)]
    pub fn rs485_add_en(&self) -> RS485_ADD_EN_R {
        RS485_ADD_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Address match value register (Low Density Only) This field contains the RS-485 address match values. Note: This field is used for RS-485 auto address detection mode."]
    #[inline(always)]
    pub fn addr_match(&self) -> ADDR_MATCH_R {
        ADDR_MATCH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART LIN Break Field Length This field indicates a 4-bit LIN TX break field count. NOTE: This break field length is UA_LIN_BKFL + 2"]
    #[inline(always)]
    pub fn ua_lin_bkfl(&mut self) -> UA_LIN_BKFL_W {
        UA_LIN_BKFL_W { w: self }
    }
    #[doc = "Bit 6 - LIN RX Enable 1 = Enable LIN RX mode. 0 = Disable LIN RX mode."]
    #[inline(always)]
    pub fn lin_rx_en(&mut self) -> LIN_RX_EN_W {
        LIN_RX_EN_W { w: self }
    }
    #[doc = "Bit 7 - LIN TX Break Mode Enable 1 = Enable LIN TX Break mode. 0 = Disable LIN TX Break mode. NOTE: When TX break field transfer operation finish, this will be cleared automatically."]
    #[inline(always)]
    pub fn lin_tx_en(&mut self) -> LIN_TX_EN_W {
        LIN_TX_EN_W { w: self }
    }
    #[doc = "Bit 8 - RS-485 Normal Multi-drop Operation Mode (NMM) (Low Density Only) 1 = Enable RS-485 Normal Multi-drop Operation Mode (NMM). 0 = Disable RS-485 Normal Multi-drop Operation Mode (NMM). Note: It can't be active with RS485_AAD operation mode."]
    #[inline(always)]
    pub fn rs485_nmm(&mut self) -> RS485_NMM_W {
        RS485_NMM_W { w: self }
    }
    #[doc = "Bit 9 - RS-485 Auto Address Detection Operation Mode (AAD) (Low Density Only) 1 = Enable RS-485 Auto Address Detection Operation Mode (AAD). 0 = Disable RS-485 Auto Address Detection Operation Mode (AAD). Note: It can't be active with RS485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485_aad(&mut self) -> RS485_AAD_W {
        RS485_AAD_W { w: self }
    }
    #[doc = "Bit 10 - RS-485 Auto Direction Mode (AUD) (Low Density Only) 1 = Enable RS-485 Auto Direction Operation Mode (AUO). 0 = Disable RS-485 Auto Direction Operation Mode (AUO). Note:This field is used for RS-485 any operation mode. Note: It can be active with RS-485_AAD or RS485_NMM operation mode."]
    #[inline(always)]
    pub fn rs485_aud(&mut self) -> RS485_AUD_W {
        RS485_AUD_W { w: self }
    }
    #[doc = "Bit 15 - RS-485 Address Detection Enable (Low Density Only) 1 = Enable address detection mode. 0 = Disable address detection mode. Note: This field is used for RS485 any operation mode."]
    #[inline(always)]
    pub fn rs485_add_en(&mut self) -> RS485_ADD_EN_W {
        RS485_ADD_EN_W { w: self }
    }
    #[doc = "Bits 24:31 - Address match value register (Low Density Only) This field contains the RS-485 address match values. Note: This field is used for RS-485 auto address detection mode."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> ADDR_MATCH_W {
        ADDR_MATCH_W { w: self }
    }
}
