#[doc = "Reader of register USB_INTSTS"]
pub type R = crate::R<u32, super::USB_INTSTS>;
#[doc = "Writer for register USB_INTSTS"]
pub type W = crate::W<u32, super::USB_INTSTS>;
#[doc = "Register USB_INTSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_INTSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUS_STS`"]
pub type BUS_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_STS`"]
pub struct BUS_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_STS_W<'a> {
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
#[doc = "Reader of field `USB_STS`"]
pub type USB_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_STS`"]
pub struct USB_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_STS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FLDET_STS`"]
pub type FLDET_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLDET_STS`"]
pub struct FLDET_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLDET_STS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WAKEUP_STS`"]
pub type WAKEUP_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP_STS`"]
pub struct WAKEUP_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_STS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EPEVT0`"]
pub type EPEVT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT0`"]
pub struct EPEVT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `EPEVT1`"]
pub type EPEVT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT1`"]
pub struct EPEVT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EPEVT2`"]
pub type EPEVT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT2`"]
pub struct EPEVT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `EPEVT3`"]
pub type EPEVT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT3`"]
pub struct EPEVT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EPEVT4`"]
pub type EPEVT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT4`"]
pub struct EPEVT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EPEVT5`"]
pub type EPEVT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEVT5`"]
pub struct EPEVT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEVT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP`"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The BUS event means that there is one of the suspense or the resume function in the bus. 1 = Bus event occurred; check USB_ATTR\\[3:0\\]
to know which kind of bus event was occurred, cleared by write 1 to USB_INTSTS\\[0\\]. 0 = No any BUS event is occurred"]
    #[inline(always)]
    pub fn bus_sts(&self) -> BUS_STS_R {
        BUS_STS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The USB event includes the Setup Token, IN Token, OUT ACK, ISO IN, or ISO OUT events in the bus. 1 = USB event occurred, check EPSTS0~5\\[2:0\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[1\\]
or EPSTS0~5 and SETUP (USB_INTSTS\\[31\\]) 0 = No any USB event is occurred"]
    #[inline(always)]
    pub fn usb_sts(&self) -> USB_STS_R {
        USB_STS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = There is attached/detached event in the USB bus and it is cleared by write 1 to USB_INTSTS\\[2\\]. 0 = There is not attached/detached event in the USB"]
    #[inline(always)]
    pub fn fldet_sts(&self) -> FLDET_STS_R {
        FLDET_STS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = Wakeup event occurred, cleared by write 1 to USB_INTSTS\\[3\\]
0 = No Wakeup event is occurred"]
    #[inline(always)]
    pub fn wakeup_sts(&self) -> WAKEUP_STS_R {
        WAKEUP_STS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1 = USB event occurred on Endpoint 0, check USB_EPSTS\\[10:8\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[16\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 0"]
    #[inline(always)]
    pub fn epevt0(&self) -> EPEVT0_R {
        EPEVT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1 = USB event occurred on Endpoint 1, check USB_EPSTS\\[13:11\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[17\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 1"]
    #[inline(always)]
    pub fn epevt1(&self) -> EPEVT1_R {
        EPEVT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1 = USB event occurred on Endpoint 2, check USB_EPSTS\\[16:14\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[18\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 2"]
    #[inline(always)]
    pub fn epevt2(&self) -> EPEVT2_R {
        EPEVT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 1 = USB event occurred on Endpoint 3, check USB_EPSTS\\[19:17\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[19\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 3"]
    #[inline(always)]
    pub fn epevt3(&self) -> EPEVT3_R {
        EPEVT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 1 = USB event occurred on Endpoint 4, check USB_EPSTS\\[22:20\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[20\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 4"]
    #[inline(always)]
    pub fn epevt4(&self) -> EPEVT4_R {
        EPEVT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1 = USB event occurred on Endpoint 5, check USB_EPSTS\\[25:23\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[21\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 5"]
    #[inline(always)]
    pub fn epevt5(&self) -> EPEVT5_R {
        EPEVT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1 = Setup event occurred, cleared by write 1 to USB_INTSTS\\[31\\]
0 = No Setup event"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The BUS event means that there is one of the suspense or the resume function in the bus. 1 = Bus event occurred; check USB_ATTR\\[3:0\\]
to know which kind of bus event was occurred, cleared by write 1 to USB_INTSTS\\[0\\]. 0 = No any BUS event is occurred"]
    #[inline(always)]
    pub fn bus_sts(&mut self) -> BUS_STS_W {
        BUS_STS_W { w: self }
    }
    #[doc = "Bit 1 - The USB event includes the Setup Token, IN Token, OUT ACK, ISO IN, or ISO OUT events in the bus. 1 = USB event occurred, check EPSTS0~5\\[2:0\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[1\\]
or EPSTS0~5 and SETUP (USB_INTSTS\\[31\\]) 0 = No any USB event is occurred"]
    #[inline(always)]
    pub fn usb_sts(&mut self) -> USB_STS_W {
        USB_STS_W { w: self }
    }
    #[doc = "Bit 2 - 1 = There is attached/detached event in the USB bus and it is cleared by write 1 to USB_INTSTS\\[2\\]. 0 = There is not attached/detached event in the USB"]
    #[inline(always)]
    pub fn fldet_sts(&mut self) -> FLDET_STS_W {
        FLDET_STS_W { w: self }
    }
    #[doc = "Bit 3 - 1 = Wakeup event occurred, cleared by write 1 to USB_INTSTS\\[3\\]
0 = No Wakeup event is occurred"]
    #[inline(always)]
    pub fn wakeup_sts(&mut self) -> WAKEUP_STS_W {
        WAKEUP_STS_W { w: self }
    }
    #[doc = "Bit 16 - 1 = USB event occurred on Endpoint 0, check USB_EPSTS\\[10:8\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[16\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 0"]
    #[inline(always)]
    pub fn epevt0(&mut self) -> EPEVT0_W {
        EPEVT0_W { w: self }
    }
    #[doc = "Bit 17 - 1 = USB event occurred on Endpoint 1, check USB_EPSTS\\[13:11\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[17\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 1"]
    #[inline(always)]
    pub fn epevt1(&mut self) -> EPEVT1_W {
        EPEVT1_W { w: self }
    }
    #[doc = "Bit 18 - 1 = USB event occurred on Endpoint 2, check USB_EPSTS\\[16:14\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[18\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 2"]
    #[inline(always)]
    pub fn epevt2(&mut self) -> EPEVT2_W {
        EPEVT2_W { w: self }
    }
    #[doc = "Bit 19 - 1 = USB event occurred on Endpoint 3, check USB_EPSTS\\[19:17\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[19\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 3"]
    #[inline(always)]
    pub fn epevt3(&mut self) -> EPEVT3_W {
        EPEVT3_W { w: self }
    }
    #[doc = "Bit 20 - 1 = USB event occurred on Endpoint 4, check USB_EPSTS\\[22:20\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[20\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 4"]
    #[inline(always)]
    pub fn epevt4(&mut self) -> EPEVT4_W {
        EPEVT4_W { w: self }
    }
    #[doc = "Bit 21 - 1 = USB event occurred on Endpoint 5, check USB_EPSTS\\[25:23\\]
to know which kind of USB event was occurred, cleared by write 1 to USB_INTSTS\\[21\\]
or USB_INTSTS\\[1\\]
0 = No event occurred in endpoint 5"]
    #[inline(always)]
    pub fn epevt5(&mut self) -> EPEVT5_W {
        EPEVT5_W { w: self }
    }
    #[doc = "Bit 31 - 1 = Setup event occurred, cleared by write 1 to USB_INTSTS\\[31\\]
0 = No Setup event"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
}
