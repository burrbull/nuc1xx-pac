#[doc = "Reader of register CAN_ERR"]
pub type R = crate::R<u32, super::CAN_ERR>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RP`"]
pub type RP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter Actual state of the Transmit Error Counter. Values between 0 and 255."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter Actual state of the Receive Error Counter. Values between 0 and 127."]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive 1 = The Receive Error Counter has reached the error passive level as defined in the CAN Specification. 0 = The Receive Error Counter is below the error passive level."]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
