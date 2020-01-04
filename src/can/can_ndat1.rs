#[doc = "Reader of register CAN_NDAT1"]
pub type R = crate::R<u32, super::CAN_NDAT1>;
#[doc = "Reader of field `NewData1_16`"]
pub type NEWDATA1_16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits 1-16 (of all Message Objects) 1 = The Message Handler or the application software has written new data into the data portion of this Message Object. 0 = No new data has been written into the data portion of this Message Object by the Message Handler since the last time this flag was cleared by the application software."]
    #[inline(always)]
    pub fn new_data1_16(&self) -> NEWDATA1_16_R {
        NEWDATA1_16_R::new((self.bits & 0xffff) as u16)
    }
}
