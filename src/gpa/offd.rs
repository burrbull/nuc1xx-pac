#[doc = "Reader of register OFFD"]
pub type R = crate::R<u32, super::OFFD>;
#[doc = "Writer for register OFFD"]
pub type W = crate::W<u32, super::OFFD>;
#[doc = "Register OFFD `reset()`'s with value 0"]
impl crate::ResetValue for super::OFFD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFD`"]
pub type OFFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFD`"]
pub struct OFFD_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - GPIOx Pin\\[n\\]
OFF digital input path Enable Each of these bits is used to control if the input path of corresponding GPIO pin is disabled. If input is analog signal, users can OFF digital input path to avoid creepage 1 = Disable IO digital input path (digital input tied to low) 0 = Enable IO digital input path"]
    #[inline(always)]
    pub fn offd(&self) -> OFFD_R {
        OFFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - GPIOx Pin\\[n\\]
OFF digital input path Enable Each of these bits is used to control if the input path of corresponding GPIO pin is disabled. If input is analog signal, users can OFF digital input path to avoid creepage 1 = Disable IO digital input path (digital input tied to low) 0 = Enable IO digital input path"]
    #[inline(always)]
    pub fn offd(&mut self) -> OFFD_W {
        OFFD_W { w: self }
    }
}
