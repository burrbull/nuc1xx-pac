#[doc = "Reader of register CAN_IIDR"]
pub type R = crate::R<u32, super::CAN_IIDR>;
#[doc = "Reader of field `IntId`"]
pub type INTID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Identifier (Indicates the source of the interrupt. Ref. Table 5-18) If several interrupts are pending, the CAN Interrupt Register will point to the pending interrupt with the highest priority, disregarding their chronological order. An interrupt remains pending until the application software has cleared it. If IntId is different from 0x0000 and IE is set, the IRQ interrupt signal to the EIC is active. The interrupt remains active until IntId is back to value 0x0000 (the cause of the interrupt is reset) or until IE is reset. The Status Interrupt has the highest priority. Among the message interrupts, the Message Object' s interrupt priority decreases with increasing message number. A message interrupt is cleared by clearing the Message Object's IntPnd bit. The Status Interrupt is cleared by reading the Status Register."]
    #[inline(always)]
    pub fn int_id(&self) -> INTID_R {
        INTID_R::new((self.bits & 0xffff) as u16)
    }
}
