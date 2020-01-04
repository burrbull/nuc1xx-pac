#[doc = "Reader of register CAN_IF1_MCON"]
pub type R = crate::R<u32, super::CAN_IF1_MCON>;
#[doc = "Writer for register CAN_IF1_MCON"]
pub type W = crate::W<u32, super::CAN_IF1_MCON>;
#[doc = "Register CAN_IF1_MCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF1_MCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLC`"]
pub type DLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLC`"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EoB`"]
pub type EOB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EoB`"]
pub struct EOB_W<'a> {
    w: &'a mut W,
}
impl<'a> EOB_W<'a> {
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
#[doc = "Reader of field `TxRqst`"]
pub type TXRQST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxRqst`"]
pub struct TXRQST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQST_W<'a> {
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
#[doc = "Reader of field `RmtEn`"]
pub type RMTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RmtEn`"]
pub struct RMTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTEN_W<'a> {
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
#[doc = "Reader of field `RxIE`"]
pub type RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxIE`"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
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
#[doc = "Reader of field `TxIE`"]
pub type TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxIE`"]
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
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
#[doc = "Reader of field `UMask`"]
pub type UMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UMask`"]
pub struct UMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UMASK_W<'a> {
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
#[doc = "Reader of field `IntPnd`"]
pub type INTPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IntPnd`"]
pub struct INTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND_W<'a> {
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
#[doc = "Reader of field `MsgLst`"]
pub type MSGLST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MsgLst`"]
pub struct MSGLST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGLST_W<'a> {
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
#[doc = "Reader of field `NewDat`"]
pub type NEWDAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NewDat`"]
pub struct NEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEWDAT_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Data Length Code 0-8: Data Frame has 0-8 data bytes. 9-15: Data Frame has 8 data bytes Note: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. Data 0: 1st data byte of a CAN Data Frame Data 1: 2nd data byte of a CAN Data Frame Data 2: 3rd data byte of a CAN Data Frame Data 3: 4th data byte of a CAN Data Frame Data 4: 5th data byte of a CAN Data Frame Data 5: 6th data byte of a CAN Data Frame Data 6: 7th data byte of a CAN Data Frame Data 7 : 8th data byte of a CAN Data Frame Note: The Data 0 Byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data 7 byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer 1 = Single Message Object or last Message Object of a FIFO Buffer. 0 = Message Object belongs to a FIFO Buffer and is not the last Message Object of that FIFO Buffer. Note: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
    #[inline(always)]
    pub fn eo_b(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Request 1 = The transmission of this Message Object is requested and is not yet done. 0 = This Message Object is not waiting for transmission."]
    #[inline(always)]
    pub fn tx_rqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remote Enable 1 = At the reception of a Remote Frame, TxRqst is set. 0 = At the reception of a Remote Frame, TxRqst is left unchanged."]
    #[inline(always)]
    pub fn rmt_en(&self) -> RMTEN_R {
        RMTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable 1 = IntPnd will be set after a successful reception of a frame. 0 = IntPnd will be left unchanged after a successful reception of a frame."]
    #[inline(always)]
    pub fn rx_ie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable 1 = IntPnd will be set after a successful transmission of a frame. 0 = IntPnd will be left unchanged after the successful transmission of a frame."]
    #[inline(always)]
    pub fn tx_ie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask 1 = Use Mask (Msk28-0, MXtd, and MDir) for acceptance filtering. 0 = Mask ignored. Note: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal is set to one."]
    #[inline(always)]
    pub fn umask(&self) -> UMASK_R {
        UMASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending 1 = This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority. 0 = This message object is not the source of an interrupt."]
    #[inline(always)]
    pub fn int_pnd(&self) -> INTPND_R {
        INTPND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Message Lost (only valid for Message Objects with direction = receive) 1 = The Message Handler stored a new message into this object when NewDat was still set, the CPU has lost a message. 0 = No message lost since last time this bit was reset by the CPU."]
    #[inline(always)]
    pub fn msg_lst(&self) -> MSGLST_R {
        MSGLST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New Data 1 = The Message Handler or the application software has written new data into the data portion of this Message Object. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the application software."]
    #[inline(always)]
    pub fn new_dat(&self) -> NEWDAT_R {
        NEWDAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code 0-8: Data Frame has 0-8 data bytes. 9-15: Data Frame has 8 data bytes Note: The Data Length Code of a Message Object must be defined the same as in all the corresponding objects with the same identifier at other nodes. When the Message Handler stores a data frame, it will write the DLC to the value given by the received message. Data 0: 1st data byte of a CAN Data Frame Data 1: 2nd data byte of a CAN Data Frame Data 2: 3rd data byte of a CAN Data Frame Data 3: 4th data byte of a CAN Data Frame Data 4: 5th data byte of a CAN Data Frame Data 5: 6th data byte of a CAN Data Frame Data 6: 7th data byte of a CAN Data Frame Data 7 : 8th data byte of a CAN Data Frame Note: The Data 0 Byte is the first data byte shifted into the shift register of the CAN Core during a reception while the Data 7 byte is the last. When the Message Handler stores a Data Frame, it will write all the eight data bytes into a Message Object. If the Data Length Code is less than 8, the remaining bytes of the Message Object will be overwritten by unspecified values."]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bit 7 - End of Buffer 1 = Single Message Object or last Message Object of a FIFO Buffer. 0 = Message Object belongs to a FIFO Buffer and is not the last Message Object of that FIFO Buffer. Note: This bit is used to concatenate two or more Message Objects (up to 32) to build a FIFO Buffer. For single Message Objects (not belonging to a FIFO Buffer), this bit must always be set to one."]
    #[inline(always)]
    pub fn eo_b(&mut self) -> EOB_W {
        EOB_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Request 1 = The transmission of this Message Object is requested and is not yet done. 0 = This Message Object is not waiting for transmission."]
    #[inline(always)]
    pub fn tx_rqst(&mut self) -> TXRQST_W {
        TXRQST_W { w: self }
    }
    #[doc = "Bit 9 - Remote Enable 1 = At the reception of a Remote Frame, TxRqst is set. 0 = At the reception of a Remote Frame, TxRqst is left unchanged."]
    #[inline(always)]
    pub fn rmt_en(&mut self) -> RMTEN_W {
        RMTEN_W { w: self }
    }
    #[doc = "Bit 10 - Receive Interrupt Enable 1 = IntPnd will be set after a successful reception of a frame. 0 = IntPnd will be left unchanged after a successful reception of a frame."]
    #[inline(always)]
    pub fn rx_ie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable 1 = IntPnd will be set after a successful transmission of a frame. 0 = IntPnd will be left unchanged after the successful transmission of a frame."]
    #[inline(always)]
    pub fn tx_ie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    #[doc = "Bit 12 - Use Acceptance Mask 1 = Use Mask (Msk28-0, MXtd, and MDir) for acceptance filtering. 0 = Mask ignored. Note: If the UMask bit is set to one, the Message Object's mask bits have to be programmed during initialization of the Message Object before MsgVal is set to one."]
    #[inline(always)]
    pub fn umask(&mut self) -> UMASK_W {
        UMASK_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Pending 1 = This message object is the source of an interrupt. The Interrupt Identifier in the Interrupt Register will point to this message object if there is no other interrupt source with higher priority. 0 = This message object is not the source of an interrupt."]
    #[inline(always)]
    pub fn int_pnd(&mut self) -> INTPND_W {
        INTPND_W { w: self }
    }
    #[doc = "Bit 14 - Message Lost (only valid for Message Objects with direction = receive) 1 = The Message Handler stored a new message into this object when NewDat was still set, the CPU has lost a message. 0 = No message lost since last time this bit was reset by the CPU."]
    #[inline(always)]
    pub fn msg_lst(&mut self) -> MSGLST_W {
        MSGLST_W { w: self }
    }
    #[doc = "Bit 15 - New Data 1 = The Message Handler or the application software has written new data into the data portion of this Message Object. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since last time this flag was cleared by the application software."]
    #[inline(always)]
    pub fn new_dat(&mut self) -> NEWDAT_W {
        NEWDAT_W { w: self }
    }
}
