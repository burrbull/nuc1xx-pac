#[doc = "Reader of register CAN_BTIME"]
pub type R = crate::R<u32, super::CAN_BTIME>;
#[doc = "Writer for register CAN_BTIME"]
pub type W = crate::W<u32, super::CAN_BTIME>;
#[doc = "Register CAN_BTIME `reset()`'s with value 0x2301"]
impl crate::ResetValue for super::CAN_BTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2301
    }
}
#[doc = "Reader of field `BRP`"]
pub type BRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRP`"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TSeg1`"]
pub type TSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSeg1`"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSeg2`"]
pub type TSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSeg2`"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler 0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width 0x0-0x3: Valid programmed values are \\[0 ... 3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment before the sample Point Minus Sync_seg 0x01-0x0F: valid values for TSeg1 are \\[1 ... 15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After sample Point 0x0-0x7: Valid values for TSeg2 are \\[0 ... 7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler 0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width 0x0-0x3: Valid programmed values are \\[0 ... 3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment before the sample Point Minus Sync_seg 0x01-0x0F: valid values for TSeg1 are \\[1 ... 15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment After sample Point 0x0-0x7: Valid values for TSeg2 are \\[0 ... 7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
}
