#[doc = "Reader of register CAN_WU_STATUS"]
pub type R = crate::R<u32, super::CAN_WU_STATUS>;
#[doc = "Writer for register CAN_WU_STATUS"]
pub type W = crate::W<u32, super::CAN_WU_STATUS>;
#[doc = "Register CAN_WU_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_WU_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKUP_STS`"]
pub type WAKUP_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKUP_STS`"]
pub struct WAKUP_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKUP_STS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wake Up Status 1 = Wake-up event is occurred. 0 = No wake-up event is occurred. Note: The bit can be written '0' to clear."]
    #[inline(always)]
    pub fn wakup_sts(&self) -> WAKUP_STS_R {
        WAKUP_STS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake Up Status 1 = Wake-up event is occurred. 0 = No wake-up event is occurred. Note: The bit can be written '0' to clear."]
    #[inline(always)]
    pub fn wakup_sts(&mut self) -> WAKUP_STS_W {
        WAKUP_STS_W { w: self }
    }
}
