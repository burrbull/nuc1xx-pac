#[doc = "Reader of register IRQ7_SRC"]
pub type R = crate::R<u32, super::IRQ7_SRC>;
#[doc = "Reader of field `INT_SRC`"]
pub type INT_SRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Bit3: PWM7_INT Bit2: PWM6_INT Bit1: PWM5_INT Bit0: PWM4_INT"]
    #[inline(always)]
    pub fn int_src(&self) -> INT_SRC_R {
        INT_SRC_R::new((self.bits & 0x07) as u8)
    }
}
