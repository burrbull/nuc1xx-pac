#[doc = "Reader of register ISPCON"]
pub type R = crate::R<u32, super::ISPCON>;
#[doc = "Writer for register ISPCON"]
pub type W = crate::W<u32, super::ISPCON>;
#[doc = "Register ISPCON `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPEN`"]
pub type ISPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISPEN`"]
pub struct ISPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPEN_W<'a> {
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
#[doc = "Reader of field `BS`"]
pub type BS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BS`"]
pub struct BS_W<'a> {
    w: &'a mut W,
}
impl<'a> BS_W<'a> {
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
#[doc = "Reader of field `CFGUEN`"]
pub type CFGUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFGUEN`"]
pub struct CFGUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGUEN_W<'a> {
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
#[doc = "Reader of field `LDUEN`"]
pub type LDUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDUEN`"]
pub struct LDUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDUEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ISPFF`"]
pub type ISPFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISPFF`"]
pub struct ISPFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ET`"]
pub type ET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ET`"]
pub struct ET_W<'a> {
    w: &'a mut W,
}
impl<'a> ET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ISP Enable ISP function enable bit. Set this bit to enable ISP function. 1 = Enable ISP function 0 = Disable ISP function"]
    #[inline(always)]
    pub fn ispen(&self) -> ISPEN_R {
        ISPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Boot Select Set/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS in Config0 after power-on reset; It keeps the same value at other reset. 1 = boot from LDROM 0 = boot from APROM"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Config-bits Update by ISP LDROM update enable bit. 1 = Enable ISP can update config-bits. 0 = Disable ISP can update config-bits."]
    #[inline(always)]
    pub fn cfguen(&self) -> CFGUEN_R {
        CFGUEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LDROM Update Enable LDROM update enable bit. 1 = LDROM can be updated when the MCU runs in APROM. 0 = LDROM can not be updated"]
    #[inline(always)]
    pub fn lduen(&self) -> LDUEN_R {
        LDUEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP Fail Flag This bit is set by hardware when a triggered ISP meets any of the following conditions: (1) APROM writes to itself. (2) LDROM writes to itself. (3) CONFIG is erased/programmed if CFGUEN is set to 0. (4) Destination address is illegal, such as over an available range. Write 1 to clear."]
    #[inline(always)]
    pub fn ispff(&self) -> ISPFF_R {
        ISPFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Flash Program Time PT\\[2\\]
PT\\[1\\]
PT\\[0\\]
Program Time (us) 0 0 0 40 0 0 1 45 0 1 0 50 0 1 1 55 1 0 0 20 1 0 1 25 1 1 0 30 1 1 1 35"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Flash Erase Time ET\\[2\\]
ET\\[1\\]
ET\\[0\\]
Erase Time (ms) 0 0 0 20 (default) 0 0 1 25 0 1 0 30 0 1 1 35 1 0 0 3 1 0 1 5 1 1 0 10 1 1 1 15"]
    #[inline(always)]
    pub fn et(&self) -> ET_R {
        ET_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ISP Enable ISP function enable bit. Set this bit to enable ISP function. 1 = Enable ISP function 0 = Disable ISP function"]
    #[inline(always)]
    pub fn ispen(&mut self) -> ISPEN_W {
        ISPEN_W { w: self }
    }
    #[doc = "Bit 1 - Boot Select Set/clear this bit to select next booting from LDROM/APROM, respectively. This bit also functions as chip booting status flag, which can be used to check where chip booted from. This bit is initiated with the inversed value of CBS in Config0 after power-on reset; It keeps the same value at other reset. 1 = boot from LDROM 0 = boot from APROM"]
    #[inline(always)]
    pub fn bs(&mut self) -> BS_W {
        BS_W { w: self }
    }
    #[doc = "Bit 4 - Enable Config-bits Update by ISP LDROM update enable bit. 1 = Enable ISP can update config-bits. 0 = Disable ISP can update config-bits."]
    #[inline(always)]
    pub fn cfguen(&mut self) -> CFGUEN_W {
        CFGUEN_W { w: self }
    }
    #[doc = "Bit 5 - LDROM Update Enable LDROM update enable bit. 1 = LDROM can be updated when the MCU runs in APROM. 0 = LDROM can not be updated"]
    #[inline(always)]
    pub fn lduen(&mut self) -> LDUEN_W {
        LDUEN_W { w: self }
    }
    #[doc = "Bit 6 - ISP Fail Flag This bit is set by hardware when a triggered ISP meets any of the following conditions: (1) APROM writes to itself. (2) LDROM writes to itself. (3) CONFIG is erased/programmed if CFGUEN is set to 0. (4) Destination address is illegal, such as over an available range. Write 1 to clear."]
    #[inline(always)]
    pub fn ispff(&mut self) -> ISPFF_W {
        ISPFF_W { w: self }
    }
    #[doc = "Bits 8:10 - Flash Program Time PT\\[2\\]
PT\\[1\\]
PT\\[0\\]
Program Time (us) 0 0 0 40 0 0 1 45 0 1 0 50 0 1 1 55 1 0 0 20 1 0 1 25 1 1 0 30 1 1 1 35"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Bits 12:14 - Flash Erase Time ET\\[2\\]
ET\\[1\\]
ET\\[0\\]
Erase Time (ms) 0 0 0 20 (default) 0 0 1 25 0 1 0 30 0 1 1 35 1 0 0 3 1 0 1 5 1 1 0 10 1 1 1 15"]
    #[inline(always)]
    pub fn et(&mut self) -> ET_W {
        ET_W { w: self }
    }
}
