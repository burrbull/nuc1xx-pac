#[doc = "Reader of register UA_BAUD"]
pub type R = crate::R<u32, super::UA_BAUD>;
#[doc = "Writer for register UA_BAUD"]
pub type W = crate::W<u32, super::UA_BAUD>;
#[doc = "Register UA_BAUD `reset()`'s with value 0x0f00_0000"]
impl crate::ResetValue for super::UA_BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0000
    }
}
#[doc = "Reader of field `BRD`"]
pub type BRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRD`"]
pub struct BRD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DIVIDER_X`"]
pub type DIVIDER_X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVIDER_X`"]
pub struct DIVIDER_X_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `DIV_X_ONE`"]
pub type DIV_X_ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_X_ONE`"]
pub struct DIV_X_ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_X_ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DIV_X_EN`"]
pub type DIV_X_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_X_EN`"]
pub struct DIV_X_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_X_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Baud Rate Divider The field indicated the baud rate divider"]
    #[inline(always)]
    pub fn brd(&self) -> BRD_R {
        BRD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Divider X The baud rate divider M = X+1."]
    #[inline(always)]
    pub fn divider_x(&self) -> DIVIDER_X_R {
        DIVIDER_X_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Divider X equal 1 0 = Divider M = X (the equation of M = X+1, but Divider_X\\[27:24\\]
must > =8) 1 = Divider M = 1 (the equation of M = 1, but BRD\\[15:0\\]
must >=3). Mode DIV_X_EN DIV_X_ONE DIVIDER X BRD Baud rate equation 0 Disable 0 B A UART_CLK / \\[16 * (A+2)\\]
1 Enable 0 B A UART_CLK/\\[(B+1)*(A+2)\\],B must >= 8 2 Enable 1 Don't Care A UART_CLK / (A+2), A must >=3"]
    #[inline(always)]
    pub fn div_x_one(&self) -> DIV_X_ONE_R {
        DIV_X_ONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Divider X Enable The BRD = Baud Rate Divider, and the baud rate equation is Baud Rate = Clock / \\[ M * (BRD + 2) \\]
; The default value of M is 16. 0 = Disable divider X (the equation of M = 16) 1 = Enable divider X (the equation of M = X+1, but Divider_X\\[27:24 must > =8). NOTE: When in IrDA mode, this bit must disable. Mode DIV_X_EN DIV_X_ONE DIVIDER X BRD Baud rate equation 0 Disable 0 B A UART_CLK / \\[16 * (A+2)\\]
1 Enable 0 B A UART_CLK/\\[(B+1)*(A+2)\\],B must >= 8 2 Enable 1 Don't Care A UART_CLK / (A+2), A must >=3"]
    #[inline(always)]
    pub fn div_x_en(&self) -> DIV_X_EN_R {
        DIV_X_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Baud Rate Divider The field indicated the baud rate divider"]
    #[inline(always)]
    pub fn brd(&mut self) -> BRD_W {
        BRD_W { w: self }
    }
    #[doc = "Bits 24:27 - Divider X The baud rate divider M = X+1."]
    #[inline(always)]
    pub fn divider_x(&mut self) -> DIVIDER_X_W {
        DIVIDER_X_W { w: self }
    }
    #[doc = "Bit 28 - Divider X equal 1 0 = Divider M = X (the equation of M = X+1, but Divider_X\\[27:24\\]
must > =8) 1 = Divider M = 1 (the equation of M = 1, but BRD\\[15:0\\]
must >=3). Mode DIV_X_EN DIV_X_ONE DIVIDER X BRD Baud rate equation 0 Disable 0 B A UART_CLK / \\[16 * (A+2)\\]
1 Enable 0 B A UART_CLK/\\[(B+1)*(A+2)\\],B must >= 8 2 Enable 1 Don't Care A UART_CLK / (A+2), A must >=3"]
    #[inline(always)]
    pub fn div_x_one(&mut self) -> DIV_X_ONE_W {
        DIV_X_ONE_W { w: self }
    }
    #[doc = "Bit 29 - Divider X Enable The BRD = Baud Rate Divider, and the baud rate equation is Baud Rate = Clock / \\[ M * (BRD + 2) \\]
; The default value of M is 16. 0 = Disable divider X (the equation of M = 16) 1 = Enable divider X (the equation of M = X+1, but Divider_X\\[27:24 must > =8). NOTE: When in IrDA mode, this bit must disable. Mode DIV_X_EN DIV_X_ONE DIVIDER X BRD Baud rate equation 0 Disable 0 B A UART_CLK / \\[16 * (A+2)\\]
1 Enable 0 B A UART_CLK/\\[(B+1)*(A+2)\\],B must >= 8 2 Enable 1 Don't Care A UART_CLK / (A+2), A must >=3"]
    #[inline(always)]
    pub fn div_x_en(&mut self) -> DIV_X_EN_W {
        DIV_X_EN_W { w: self }
    }
}
