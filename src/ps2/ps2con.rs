#[doc = "Reader of register PS2CON"]
pub type R = crate::R<u32, super::PS2CON>;
#[doc = "Writer for register PS2CON"]
pub type W = crate::W<u32, super::PS2CON>;
#[doc = "Register PS2CON `reset()`'s with value 0"]
impl crate::ResetValue for super::PS2CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PS2EN`"]
pub type PS2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PS2EN`"]
pub struct PS2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2EN_W<'a> {
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
#[doc = "Reader of field `TXINTEN`"]
pub type TXINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINTEN`"]
pub struct TXINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINTEN_W<'a> {
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
#[doc = "Reader of field `RXINTEN`"]
pub type RXINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINTEN`"]
pub struct RXINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTEN_W<'a> {
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
#[doc = "Reader of field `TXFIFO_DEPTH`"]
pub type TXFIFO_DEPTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFIFO_DEPTH`"]
pub struct TXFIFO_DEPTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_DEPTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CLRFIFO`"]
pub type CLRFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRFIFO`"]
pub struct CLRFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRFIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `OVERRIDE`"]
pub type OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRIDE`"]
pub struct OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FPS2CLK`"]
pub type FPS2CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPS2CLK`"]
pub struct FPS2CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FPS2CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FPS2DAT`"]
pub type FPS2DAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPS2DAT`"]
pub struct FPS2DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FPS2DAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable PS2 Device Enable PS2 device controller 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn ps2en(&self) -> PS2EN_R {
        PS2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Interrupt 1 = Enable data transmit complete interrupt 0 = Disable data transmit complete interrupt"]
    #[inline(always)]
    pub fn txinten(&self) -> TXINTEN_R {
        TXINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Receive Interrupt 1 = Enable data receive complete interrupt 0 = Disable data receive complete interrupt"]
    #[inline(always)]
    pub fn rxinten(&self) -> RXINTEN_R {
        RXINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - Transmit Data FIFO Depth There is 16 bytes buffer for data transmit. S/W can define the FIFO depth from 1 to 16 bytes depends on application. 0 = 1 byte 1 = 2 bytes ... 14 = 15 bytes 15 = 16 bytes"]
    #[inline(always)]
    pub fn txfifo_depth(&self) -> TXFIFO_DEPTH_R {
        TXFIFO_DEPTH_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Acknowledge Enable 1 = If parity error or stop bit is not received correctly, acknowledge bit will not be sent to host at 12th clock 0 = Always send acknowledge to host at 12th clock for host to device communication."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear TX FIFO Write 1 to this bit to terminate device to host transmission. The TXEMPTY bit in PS2STATUS bit will be set to 1 and pointer BYTEIDEX is reset to 0 regardless there is residue data in buffer or not. The buffer content is not been cleared. 1 = Clear FIFO 0 = Not active"]
    #[inline(always)]
    pub fn clrfifo(&self) -> CLRFIFO_R {
        CLRFIFO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software Override PS2 CLK/DATA Pin State 1 = PS2CLK and PS2DATA pins are controlled by S/W 0 = PS2CLK and PS2DATA pins are controlled by internal state machine."]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force PS2CLK Line It forces PS2CLK line high or low regardless of the internal state of the device controller if OVERRIDE is set to high. 1 = Force PS2DATA line high 0 = Force PS2DATA line low"]
    #[inline(always)]
    pub fn fps2clk(&self) -> FPS2CLK_R {
        FPS2CLK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force PS2DATA Line It forces PS2DATA high or low regardless of the internal state of the device controller if OVERRIDE is set to high. 1 = Force PS2DATA high 0 = Force PS2DATA low"]
    #[inline(always)]
    pub fn fps2dat(&self) -> FPS2DAT_R {
        FPS2DAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PS2 Device Enable PS2 device controller 1 = Enable 0 = Disable"]
    #[inline(always)]
    pub fn ps2en(&mut self) -> PS2EN_W {
        PS2EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Transmit Interrupt 1 = Enable data transmit complete interrupt 0 = Disable data transmit complete interrupt"]
    #[inline(always)]
    pub fn txinten(&mut self) -> TXINTEN_W {
        TXINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable Receive Interrupt 1 = Enable data receive complete interrupt 0 = Disable data receive complete interrupt"]
    #[inline(always)]
    pub fn rxinten(&mut self) -> RXINTEN_W {
        RXINTEN_W { w: self }
    }
    #[doc = "Bits 3:6 - Transmit Data FIFO Depth There is 16 bytes buffer for data transmit. S/W can define the FIFO depth from 1 to 16 bytes depends on application. 0 = 1 byte 1 = 2 bytes ... 14 = 15 bytes 15 = 16 bytes"]
    #[inline(always)]
    pub fn txfifo_depth(&mut self) -> TXFIFO_DEPTH_W {
        TXFIFO_DEPTH_W { w: self }
    }
    #[doc = "Bit 7 - Acknowledge Enable 1 = If parity error or stop bit is not received correctly, acknowledge bit will not be sent to host at 12th clock 0 = Always send acknowledge to host at 12th clock for host to device communication."]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 8 - Clear TX FIFO Write 1 to this bit to terminate device to host transmission. The TXEMPTY bit in PS2STATUS bit will be set to 1 and pointer BYTEIDEX is reset to 0 regardless there is residue data in buffer or not. The buffer content is not been cleared. 1 = Clear FIFO 0 = Not active"]
    #[inline(always)]
    pub fn clrfifo(&mut self) -> CLRFIFO_W {
        CLRFIFO_W { w: self }
    }
    #[doc = "Bit 9 - Software Override PS2 CLK/DATA Pin State 1 = PS2CLK and PS2DATA pins are controlled by S/W 0 = PS2CLK and PS2DATA pins are controlled by internal state machine."]
    #[inline(always)]
    pub fn override_(&mut self) -> OVERRIDE_W {
        OVERRIDE_W { w: self }
    }
    #[doc = "Bit 10 - Force PS2CLK Line It forces PS2CLK line high or low regardless of the internal state of the device controller if OVERRIDE is set to high. 1 = Force PS2DATA line high 0 = Force PS2DATA line low"]
    #[inline(always)]
    pub fn fps2clk(&mut self) -> FPS2CLK_W {
        FPS2CLK_W { w: self }
    }
    #[doc = "Bit 11 - Force PS2DATA Line It forces PS2DATA high or low regardless of the internal state of the device controller if OVERRIDE is set to high. 1 = Force PS2DATA high 0 = Force PS2DATA low"]
    #[inline(always)]
    pub fn fps2dat(&mut self) -> FPS2DAT_W {
        FPS2DAT_W { w: self }
    }
}
