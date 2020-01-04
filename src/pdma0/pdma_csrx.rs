#[doc = "Reader of register PDMA_CSRx"]
pub type R = crate::R<u32, super::PDMA_CSRX>;
#[doc = "Writer for register PDMA_CSRx"]
pub type W = crate::W<u32, super::PDMA_CSRX>;
#[doc = "Register PDMA_CSRx `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_CSRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMACEN`"]
pub type PDMACEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMACEN`"]
pub struct PDMACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMACEN_W<'a> {
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
#[doc = "Reader of field `SW_RST`"]
pub type SW_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST`"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
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
#[doc = "Reader of field `MODE_SEL`"]
pub type MODE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE_SEL`"]
pub struct MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SAD_SEL`"]
pub type SAD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAD_SEL`"]
pub struct SAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DAD_SEL`"]
pub type DAD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAD_SEL`"]
pub struct DAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `APB_TWS`"]
pub type APB_TWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_TWS`"]
pub struct APB_TWS_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_TWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `TRIG_EN`"]
pub type TRIG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG_EN`"]
pub struct TRIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Channel Enable Setting this bit to 1 enables PDMA's operation. If this bit is cleared, PDMA will ignore all PDMA request and force Bus Master into IDLE state. Note: SW_RST(PDMA_CSRx\\[1\\], x= 0~8) will clear this bit."]
    #[inline(always)]
    pub fn pdmacen(&self) -> PDMACEN_R {
        PDMACEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Engine Reset 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the internal state machine and pointers. The contents of control register will not be cleared. This bit will auto clear after few clock cycles."]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - PDMA Mode Select 00 = Memory to Memory mode (Memory-to-Memory). 01 = IP to Memory mode (APB-to-Memory). 10 = Memory to IP mode (Memory-to-APB)."]
    #[inline(always)]
    pub fn mode_sel(&self) -> MODE_SEL_R {
        MODE_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Transfer Source Address Direction Select 00 = Transfer Source address is incremented successively. 01 = Reserved. 10 = Transfer Source address is fixed (This feature can be used when data where transferred from a single source to multiple destinations). 11 = Reserved."]
    #[inline(always)]
    pub fn sad_sel(&self) -> SAD_SEL_R {
        SAD_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Transfer Destination Address Direction Select 00 = Transfer Destination address is incremented successively. 01 = Reserved. 10 = Transfer Destination address is fixed (This feature can be used when data where transferred from multiple sources to a single destination). 11 = Reserved."]
    #[inline(always)]
    pub fn dad_sel(&self) -> DAD_SEL_R {
        DAD_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Peripheral transfer Width Select 00 = One word (32 bits) is transferred for every PDMA operation. 01 = One byte (8 bits) is transferred for every PDMA operation. 10 = One half-word (16 bits) is transferred for every PDMA operation. 11 = Reserved. Note: This field is meaningful only when MODE_SEL is IP to Memory mode (APB-to-Memory) or Memory to IP mode (Memory-to-APB)."]
    #[inline(always)]
    pub fn apb_tws(&self) -> APB_TWS_R {
        APB_TWS_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Trig_EN 0 = No effect. 1 = Enable PDMA data read or write transfer. Note: When PDMA transfer completed, this bit will be cleared automatically. If the bus error occurs, all PDMA transfer will be stopped. Software must reset all PDMA channel, and then trigger again."]
    #[inline(always)]
    pub fn trig_en(&self) -> TRIG_EN_R {
        TRIG_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Channel Enable Setting this bit to 1 enables PDMA's operation. If this bit is cleared, PDMA will ignore all PDMA request and force Bus Master into IDLE state. Note: SW_RST(PDMA_CSRx\\[1\\], x= 0~8) will clear this bit."]
    #[inline(always)]
    pub fn pdmacen(&mut self) -> PDMACEN_W {
        PDMACEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Engine Reset 0 = Writing 0 to this bit has no effect. 1 = Writing 1 to this bit will reset the internal state machine and pointers. The contents of control register will not be cleared. This bit will auto clear after few clock cycles."]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bits 2:3 - PDMA Mode Select 00 = Memory to Memory mode (Memory-to-Memory). 01 = IP to Memory mode (APB-to-Memory). 10 = Memory to IP mode (Memory-to-APB)."]
    #[inline(always)]
    pub fn mode_sel(&mut self) -> MODE_SEL_W {
        MODE_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Transfer Source Address Direction Select 00 = Transfer Source address is incremented successively. 01 = Reserved. 10 = Transfer Source address is fixed (This feature can be used when data where transferred from a single source to multiple destinations). 11 = Reserved."]
    #[inline(always)]
    pub fn sad_sel(&mut self) -> SAD_SEL_W {
        SAD_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Transfer Destination Address Direction Select 00 = Transfer Destination address is incremented successively. 01 = Reserved. 10 = Transfer Destination address is fixed (This feature can be used when data where transferred from multiple sources to a single destination). 11 = Reserved."]
    #[inline(always)]
    pub fn dad_sel(&mut self) -> DAD_SEL_W {
        DAD_SEL_W { w: self }
    }
    #[doc = "Bits 19:20 - Peripheral transfer Width Select 00 = One word (32 bits) is transferred for every PDMA operation. 01 = One byte (8 bits) is transferred for every PDMA operation. 10 = One half-word (16 bits) is transferred for every PDMA operation. 11 = Reserved. Note: This field is meaningful only when MODE_SEL is IP to Memory mode (APB-to-Memory) or Memory to IP mode (Memory-to-APB)."]
    #[inline(always)]
    pub fn apb_tws(&mut self) -> APB_TWS_W {
        APB_TWS_W { w: self }
    }
    #[doc = "Bit 23 - Trig_EN 0 = No effect. 1 = Enable PDMA data read or write transfer. Note: When PDMA transfer completed, this bit will be cleared automatically. If the bus error occurs, all PDMA transfer will be stopped. Software must reset all PDMA channel, and then trigger again."]
    #[inline(always)]
    pub fn trig_en(&mut self) -> TRIG_EN_W {
        TRIG_EN_W { w: self }
    }
}
