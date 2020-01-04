#[doc = "Reader of register USB_EPSTS"]
pub type R = crate::R<u32, super::USB_EPSTS>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPSTS0`"]
pub type EPSTS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPSTS1`"]
pub type EPSTS1_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPSTS2`"]
pub type EPSTS2_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPSTS3`"]
pub type EPSTS3_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPSTS4`"]
pub type EPSTS4_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPSTS5`"]
pub type EPSTS5_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 7 - It indicates that the received data is over the maximum payload number or not. 1 = It indicates that the Out Data more than the Max Payload in MXPLD register or the Setup Data more than 8 Bytes 0 = No overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts0(&self) -> EPSTS0_R {
        EPSTS0_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts1(&self) -> EPSTS1_R {
        EPSTS1_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts2(&self) -> EPSTS2_R {
        EPSTS2_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts3(&self) -> EPSTS3_R {
        EPSTS3_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts4(&self) -> EPSTS4_R {
        EPSTS4_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - These bits are used to indicate the current status of this endpoint 000 = In ACK 001 = In NAK 010 = Out Packet Data0 ACK 110 = Out Packet Data1 ACK 011 = Setup ACK 111 = Isochronous transfer end"]
    #[inline(always)]
    pub fn epsts5(&self) -> EPSTS5_R {
        EPSTS5_R::new(((self.bits >> 23) & 0x07) as u8)
    }
}
