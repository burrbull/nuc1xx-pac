#[doc = "Reader of register CAN_MVLD2"]
pub type R = crate::R<u32, super::CAN_MVLD2>;
#[doc = "Reader of field `MsgVal17_32`"]
pub type MSGVAL17_32_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Message Valid Bits 17-32 (of all Message Objects) (Read Only) 1 = This Message Object is configured and should be considered by the Message Handler. 0 = This Message Object is ignored by the Message Handler. Ex. CAN_MVLD1\\[0\\]
means Message object No.1 is valid or not. If CAN_MVLD1\\[0\\]
is set, message object No.1 is configured."]
    #[inline(always)]
    pub fn msg_val17_32(&self) -> MSGVAL17_32_R {
        MSGVAL17_32_R::new((self.bits & 0xffff) as u16)
    }
}
