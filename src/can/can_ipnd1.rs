#[doc = "Reader of register CAN_IPND1"]
pub type R = crate::R<u32, super::CAN_IPND1>;
#[doc = "Reader of field `IntPnd1_16`"]
pub type INTPND1_16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits 1-16 (of all Message Objects) 1 = This message object is the source of an interrupt. 0 = This message object is not the source of an interrupt."]
    #[inline(always)]
    pub fn int_pnd1_16(&self) -> INTPND1_16_R {
        INTPND1_16_R::new((self.bits & 0xffff) as u16)
    }
}
