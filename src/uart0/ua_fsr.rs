#[doc = "Reader of register UA_FSR"]
pub type R = crate::R<u32, super::UA_FSR>;
#[doc = "Writer for register UA_FSR"]
pub type W = crate::W<u32, super::UA_FSR>;
#[doc = "Register UA_FSR `reset()`'s with value 0x1040_4000"]
impl crate::ResetValue for super::UA_FSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1040_4000
    }
}
#[doc = "Reader of field `RX_OVER_IF`"]
pub type RX_OVER_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RS485_ADD_DETF`"]
pub type RS485_ADD_DETF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEF`"]
pub type PEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIF`"]
pub type BIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_POINTER`"]
pub type RX_POINTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FULL`"]
pub type RX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_POINTER`"]
pub type TX_POINTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_EMPTY`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FULL`"]
pub type TX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_OVER_IF`"]
pub type TX_OVER_IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TE_FLAG`"]
pub type TE_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Rx overflow Error IF (Read Only) This bit is set when Rx FIFO overflow. If the number of bytes of received data is greater than Rx FIFO(UA_RBR) size, 64/16 bytes of (UA_RBR), this bit will be set. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn rx_over_if(&self) -> RX_OVER_IF_R {
        RX_OVER_IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - RS-485 Address Byte Detection Flag (Read Only) (Low Density Only) This bit is set to logic 1 and set UA_ALT_CSR \\[RS485_ADD_EN\\]
whenever in RS-485 mode the receiver detect any address byte received address byte character (bit9 = 1) bit, and it is reset whenever the CPU writes 1 to this bit. NOTE: This field is used for RS-485 function mode. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn rs485_add_detf(&self) -> RS485_ADD_DETF_R {
        RS485_ADD_DETF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Flag This bit is set to logic 1 whenever the received character does not have a valid \"parity bit\", and is reset whenever the CPU writes 1 to this bit. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Framing Error Flag This bit is set to logic 1 whenever the received character does not have a valid \"stop bit\" (that is, the stop bit following the last data bit or parity bit is detected as a logic 0), and is reset whenever the CPU writes 1 to this bit. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Break Interrupt Flag This bit is set to a logic 1 whenever the received data input(Rx) is held in the \"spacing state\" (logic 0) for longer than a full word transmission time (that is, the total time of \"start bit\" + data bits + parity + stop bits) and is reset whenever the CPU writes 1 to this bit. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Rx FIFO pointer (Read Only) This field indicates the Rx FIFO Buffer Pointer. When UART receives one byte from external device, Rx_Pointer increases one. When one byte of Rx FIFO is read by CPU, Rx_Pointer decreases one."]
    #[inline(always)]
    pub fn rx_pointer(&self) -> RX_POINTER_R {
        RX_POINTER_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Receiver FIFO Empty (Read Only) This bit initiate Rx FIFO empty or not. When the last byte of Rx FIFO has been read by CPU, hardware sets this bit high. It will be cleared when UART receives any new data."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receiver FIFO Full (Read Only) This bit initiates Rx FIFO full or not. This bit is set when RX_POINTER is equal to 64/16(UART0/UART1), otherwise is cleared by hardware."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - TX FIFO Pointer (Read Only) This field indicates the Tx FIFO Buffer Pointer. When CPU write one byte into UA_THR, Tx_Pointer increases one. When one byte of Tx FIFO is transferred to Transmitter Shift Register, Tx_Pointer decreases one."]
    #[inline(always)]
    pub fn tx_pointer(&self) -> TX_POINTER_R {
        TX_POINTER_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Transmitter FIFO Empty (Read Only) This bit indicates Tx FIFO empty or not. When the last byte of Tx FIFO has been transferred to Transmitter Shift Register, hardware sets this bit high. It will be cleared when writing data into THR (Tx FIFO not empty)."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmitter FIFO Full (Read Only) This bit indicates Tx FIFO full or not. This bit is set when Tx_Point is equal to 64/16(UART0/UART1), otherwise is cleared by hardware."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tx Overflow Error Interrupt Flag (Read Only) If Tx FIFO(UA_THR) is full, an additional write to UA_THR will cause this bit to logic 1. NOTE: This bit is read only, but can be cleared by writing '1' to it."]
    #[inline(always)]
    pub fn tx_over_if(&self) -> TX_OVER_IF_R {
        TX_OVER_IF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmitter Empty Flag (Read Only) Bit is set by hardware when Tx FIFO(UA_THR) is empty and the STOP bit of the last byte has been transmitted. Bit is cleared automatically when Tx FIFO is not empty or the last byte transmission has not completed. NOTE: This bit is read only."]
    #[inline(always)]
    pub fn te_flag(&self) -> TE_FLAG_R {
        TE_FLAG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {}
