#[doc = "Reader of register CAN_IPND2"]
pub type R = crate::R<u32, super::CAN_IPND2>;
#[doc = "Reader of field `IntPnd17_32`"]
pub type INTPND17_32_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits 17-32 (of all Message Objects) 1 = This message object is the source of an interrupt. 0 = This message object is not the source of an interrupt."]
    #[inline(always)]
    pub fn int_pnd17_32(&self) -> INTPND17_32_R {
        INTPND17_32_R::new((self.bits & 0xffff) as u16)
    }
}
