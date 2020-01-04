#[doc = "Reader of register UA_FCR"]
pub type R = crate::R<u32, super::UA_FCR>;
#[doc = "Writer for register UA_FCR"]
pub type W = crate::W<u32, super::UA_FCR>;
#[doc = "Register UA_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::UA_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFR`"]
pub type RFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFR`"]
pub struct RFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFR_W<'a> {
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
#[doc = "Reader of field `TFR`"]
pub type TFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFR`"]
pub struct TFR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFR_W<'a> {
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
#[doc = "Reader of field `RFITL`"]
pub type RFITL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFITL`"]
pub struct RFITL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFITL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RX_DIS`"]
pub type RX_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DIS`"]
pub struct RX_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DIS_W<'a> {
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
#[doc = "Reader of field `RTS_TRI_LEV`"]
pub type RTS_TRI_LEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTS_TRI_LEV`"]
pub struct RTS_TRI_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_TRI_LEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Rx Field Software Reset When Rx_RST is set, all the bytes in the transmit FIFO and Rx internal state machine are cleared. 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the Rx internal state machine and pointers. Note: This bit will auto clear needs at least 3 UART engine clock cycles."]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tx Field Software Reset When Tx_RST is set, all the bytes in the transmit FIFO and Tx internal state machine are cleared. 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the Tx internal state machine and pointers. Note: This bit will auto clear needs at least 3 UART engine clock cycles."]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Rx FIFO Interrupt (INT_RDA) Trigger Level When the number of bytes in the receive FIFO equals the RFITL then the RDA_IF will be set (if IER \\[RDA_IEN\\]
is enable, an interrupt will generated). RFITL INTR_RDA Trigger Level (Bytes) 0000 01 0001 04 0010 08 0011 14 0100 30/14 (High Speed/Normal Speed) 0101 46/14 (High Speed/Normal Speed) 0110 62/14 (High Speed/Normal Speed) others 62/14 (High Speed/Normal Speed)"]
    #[inline(always)]
    pub fn rfitl(&self) -> RFITL_R {
        RFITL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Receiver Disable register The receiver is disabled or not (set 1 is disable receiver) 1 = Disable Receiver. 0 = Enable Receiver. Note: This field is used for RS-485 Normal Multi-drop mode. It should be programmed before UA_ALT_CSR \\[RS-485_NMM\\]
is programmed."]
    #[inline(always)]
    pub fn rx_dis(&self) -> RX_DIS_R {
        RX_DIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RTS Trigger Level for Auto-flow Control Use(not available in UART2 channel) RTS_Tri_Lev Trigger Level (Bytes) 0000 01 0001 04 0010 08 0011 14 0100 30/14 (High Speed/Normal Speed) 0101 46/14 (High Speed/Normal Speed) 0110 62/14 (High Speed/Normal Speed) others 62/14 (High Speed/Normal Speed)"]
    #[inline(always)]
    pub fn rts_tri_lev(&self) -> RTS_TRI_LEV_R {
        RTS_TRI_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Rx Field Software Reset When Rx_RST is set, all the bytes in the transmit FIFO and Rx internal state machine are cleared. 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the Rx internal state machine and pointers. Note: This bit will auto clear needs at least 3 UART engine clock cycles."]
    #[inline(always)]
    pub fn rfr(&mut self) -> RFR_W {
        RFR_W { w: self }
    }
    #[doc = "Bit 2 - Tx Field Software Reset When Tx_RST is set, all the bytes in the transmit FIFO and Tx internal state machine are cleared. 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the Tx internal state machine and pointers. Note: This bit will auto clear needs at least 3 UART engine clock cycles."]
    #[inline(always)]
    pub fn tfr(&mut self) -> TFR_W {
        TFR_W { w: self }
    }
    #[doc = "Bits 4:7 - Rx FIFO Interrupt (INT_RDA) Trigger Level When the number of bytes in the receive FIFO equals the RFITL then the RDA_IF will be set (if IER \\[RDA_IEN\\]
is enable, an interrupt will generated). RFITL INTR_RDA Trigger Level (Bytes) 0000 01 0001 04 0010 08 0011 14 0100 30/14 (High Speed/Normal Speed) 0101 46/14 (High Speed/Normal Speed) 0110 62/14 (High Speed/Normal Speed) others 62/14 (High Speed/Normal Speed)"]
    #[inline(always)]
    pub fn rfitl(&mut self) -> RFITL_W {
        RFITL_W { w: self }
    }
    #[doc = "Bit 8 - Receiver Disable register The receiver is disabled or not (set 1 is disable receiver) 1 = Disable Receiver. 0 = Enable Receiver. Note: This field is used for RS-485 Normal Multi-drop mode. It should be programmed before UA_ALT_CSR \\[RS-485_NMM\\]
is programmed."]
    #[inline(always)]
    pub fn rx_dis(&mut self) -> RX_DIS_W {
        RX_DIS_W { w: self }
    }
    #[doc = "Bits 16:19 - RTS Trigger Level for Auto-flow Control Use(not available in UART2 channel) RTS_Tri_Lev Trigger Level (Bytes) 0000 01 0001 04 0010 08 0011 14 0100 30/14 (High Speed/Normal Speed) 0101 46/14 (High Speed/Normal Speed) 0110 62/14 (High Speed/Normal Speed) others 62/14 (High Speed/Normal Speed)"]
    #[inline(always)]
    pub fn rts_tri_lev(&mut self) -> RTS_TRI_LEV_W {
        RTS_TRI_LEV_W { w: self }
    }
}
