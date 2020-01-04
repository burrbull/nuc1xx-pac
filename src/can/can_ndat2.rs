#[doc = "Reader of register CAN_NDAT2"]
pub type R = crate::R<u32, super::CAN_NDAT2>;
#[doc = "Reader of field `NewData17_32`"]
pub type NEWDATA17_32_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits 17-32 (of all Message Objects) 1 = The Message Handler or the application software has written new data into the data portion of this Message Object. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since the last time this flag was cleared by the application software."]
    #[inline(always)]
    pub fn new_data17_32(&self) -> NEWDATA17_32_R {
        NEWDATA17_32_R::new((self.bits & 0xffff) as u16)
    }
}
