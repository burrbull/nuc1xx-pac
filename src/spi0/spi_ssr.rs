#[doc = "Reader of register SPI_SSR"]
pub type R = crate::R<u32, super::SPI_SSR>;
#[doc = "Writer for register SPI_SSR"]
pub type W = crate::W<u32, super::SPI_SSR>;
#[doc = "Register SPI_SSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSR`"]
pub type SSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSR`"]
pub struct SSR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SS_LVL`"]
pub type SS_LVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS_LVL`"]
pub struct SS_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_LVL_W<'a> {
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
#[doc = "Reader of field `AUTOSS`"]
pub type AUTOSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSS`"]
pub struct AUTOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSS_W<'a> {
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
#[doc = "Reader of field `SS_LTRIG`"]
pub type SS_LTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS_LTRIG`"]
pub struct SS_LTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_LTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LTRIG_FLAG`"]
pub type LTRIG_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Slave Select Register (master only) If AUTOSS bit is cleared, writing 1 to any bit location of this field sets the proper SPISSx0/1 line to an active state and writing 0 sets the line back to inactive state. If AUTOSS bit is set, writing 1 to any bit location of this field will select appropriate SPISSx0/1 line to be automatically driven to active state for the duration of the transmit/receive, and will be driven to inactive state for the rest of the time. (The active level of SPISSx0/1 is specified in SS_LVL). Note: 1. This interface can only drive one device/slave at a given time. Therefore, the slave select pin of the selected device must be set to its active level before starting any read or write transfer. 2. SPISSx0 is also defined as device/slave select input signal in slave mode."]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Slave Select Active Level It defines the active level of slave select signal (SPISSx0/1). 1 = The slave select signal SPISSx0/1 is active at high-level/rising-edge. 0 = The slave select signal SPISSx0/1 is active at low-level/falling-edge.."]
    #[inline(always)]
    pub fn ss_lvl(&self) -> SS_LVL_R {
        SS_LVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Automatic Slave Select (master only) 1 = If this bit is set, SPISSx0/1 signals are generated automatically. It means that slave select signal, which is set in SSR\\[1:0\\]
register is asserted by the SPI controller when transmit/receive is started by setting GO_BUSY, and is de-asserted after each transmit/receive is finished. 0 = If this bit is cleared, slave select signals are asserted and de-asserted by setting and clearing related bits in SSR\\[1:0\\]
register."]
    #[inline(always)]
    pub fn autoss(&self) -> AUTOSS_R {
        AUTOSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select Level Trigger (slave only) 1: The slave select signal will be level-trigger. It depends on SS_LVL to decide the signal is active low or active high. 0: The input slave select signal is edge-trigger. This is default value."]
    #[inline(always)]
    pub fn ss_ltrig(&self) -> SS_LTRIG_R {
        SS_LTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Level Trigger Flag When the SS_LTRIG bit is set in slave mode, this bit can be read to indicate the received bit number is met the requirement or not. 1 = The transaction number and the transferred bit length met the specified requirements which defined in TX_NUM and TX_BIT_LEN. 0 = The transaction number or the transferred bit length of one transaction doesn't meet the specified requirements. Note: This bit is READ only"]
    #[inline(always)]
    pub fn ltrig_flag(&self) -> LTRIG_FLAG_R {
        LTRIG_FLAG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slave Select Register (master only) If AUTOSS bit is cleared, writing 1 to any bit location of this field sets the proper SPISSx0/1 line to an active state and writing 0 sets the line back to inactive state. If AUTOSS bit is set, writing 1 to any bit location of this field will select appropriate SPISSx0/1 line to be automatically driven to active state for the duration of the transmit/receive, and will be driven to inactive state for the rest of the time. (The active level of SPISSx0/1 is specified in SS_LVL). Note: 1. This interface can only drive one device/slave at a given time. Therefore, the slave select pin of the selected device must be set to its active level before starting any read or write transfer. 2. SPISSx0 is also defined as device/slave select input signal in slave mode."]
    #[inline(always)]
    pub fn ssr(&mut self) -> SSR_W {
        SSR_W { w: self }
    }
    #[doc = "Bit 2 - Slave Select Active Level It defines the active level of slave select signal (SPISSx0/1). 1 = The slave select signal SPISSx0/1 is active at high-level/rising-edge. 0 = The slave select signal SPISSx0/1 is active at low-level/falling-edge.."]
    #[inline(always)]
    pub fn ss_lvl(&mut self) -> SS_LVL_W {
        SS_LVL_W { w: self }
    }
    #[doc = "Bit 3 - Automatic Slave Select (master only) 1 = If this bit is set, SPISSx0/1 signals are generated automatically. It means that slave select signal, which is set in SSR\\[1:0\\]
register is asserted by the SPI controller when transmit/receive is started by setting GO_BUSY, and is de-asserted after each transmit/receive is finished. 0 = If this bit is cleared, slave select signals are asserted and de-asserted by setting and clearing related bits in SSR\\[1:0\\]
register."]
    #[inline(always)]
    pub fn autoss(&mut self) -> AUTOSS_W {
        AUTOSS_W { w: self }
    }
    #[doc = "Bit 4 - Slave Select Level Trigger (slave only) 1: The slave select signal will be level-trigger. It depends on SS_LVL to decide the signal is active low or active high. 0: The input slave select signal is edge-trigger. This is default value."]
    #[inline(always)]
    pub fn ss_ltrig(&mut self) -> SS_LTRIG_W {
        SS_LTRIG_W { w: self }
    }
}
