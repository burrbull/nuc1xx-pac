#[doc = "Reader of register EXTIME"]
pub type R = crate::R<u32, super::EXTIME>;
#[doc = "Writer for register EXTIME"]
pub type W = crate::W<u32, super::EXTIME>;
#[doc = "Register EXTIME `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ExttACC`"]
pub type EXTTACC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ExttACC`"]
pub struct EXTTACC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `ExttAHD`"]
pub type EXTTAHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ExttAHD`"]
pub struct EXTTAHD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTAHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ExtIW2X`"]
pub type EXTIW2X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ExtIW2X`"]
pub struct EXTIW2X_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIW2X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ExtIR2R`"]
pub type EXTIR2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ExtIR2R`"]
pub struct EXTIR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIR2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - EBI Data Access Time ExttACC define data access time (tACC). tACC = (ExttACC+1)*MCLK"]
    #[inline(always)]
    pub fn extt_acc(&self) -> EXTTACC_R {
        EXTTACC_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - EBI Data Access Hold Time ExttAHD define data access hold time (tAHD). tAHD = (ExttAHD+1)*MCLK"]
    #[inline(always)]
    pub fn extt_ahd(&self) -> EXTTAHD_R {
        EXTTAHD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Idle State Cycle After Write When write action is finish, idle state is inserted and nCS return to high if ExtIW2X is not zero. Idle state cycle = (ExtIW2X)*MCLK"]
    #[inline(always)]
    pub fn ext_iw2x(&self) -> EXTIW2X_R {
        EXTIW2X_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Idle State Cycle Between Read-Read When read action is finish and next action is going to read, idle state is inserted and nCS return to high if ExtIR2R is not zero. Idle state cycle = (ExtIR2R)*MCLK"]
    #[inline(always)]
    pub fn ext_ir2r(&self) -> EXTIR2R_R {
        EXTIR2R_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - EBI Data Access Time ExttACC define data access time (tACC). tACC = (ExttACC+1)*MCLK"]
    #[inline(always)]
    pub fn extt_acc(&mut self) -> EXTTACC_W {
        EXTTACC_W { w: self }
    }
    #[doc = "Bits 8:10 - EBI Data Access Hold Time ExttAHD define data access hold time (tAHD). tAHD = (ExttAHD+1)*MCLK"]
    #[inline(always)]
    pub fn extt_ahd(&mut self) -> EXTTAHD_W {
        EXTTAHD_W { w: self }
    }
    #[doc = "Bits 12:15 - Idle State Cycle After Write When write action is finish, idle state is inserted and nCS return to high if ExtIW2X is not zero. Idle state cycle = (ExtIW2X)*MCLK"]
    #[inline(always)]
    pub fn ext_iw2x(&mut self) -> EXTIW2X_W {
        EXTIW2X_W { w: self }
    }
    #[doc = "Bits 24:27 - Idle State Cycle Between Read-Read When read action is finish and next action is going to read, idle state is inserted and nCS return to high if ExtIR2R is not zero. Idle state cycle = (ExtIR2R)*MCLK"]
    #[inline(always)]
    pub fn ext_ir2r(&mut self) -> EXTIR2R_W {
        EXTIR2R_W { w: self }
    }
}
