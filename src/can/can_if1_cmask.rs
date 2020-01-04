#[doc = "Reader of register CAN_IF1_CMASK"]
pub type R = crate::R<u32, super::CAN_IF1_CMASK>;
#[doc = "Writer for register CAN_IF1_CMASK"]
pub type W = crate::W<u32, super::CAN_IF1_CMASK>;
#[doc = "Register CAN_IF1_CMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::CAN_IF1_CMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT_B`"]
pub type DAT_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_B`"]
pub struct DAT_B_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_B_W<'a> {
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
#[doc = "Reader of field `DAT_A`"]
pub type DAT_A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAT_A`"]
pub struct DAT_A_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_A_W<'a> {
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
#[doc = "Reader of field `TxRqstOrNewDat`"]
pub type TXRQSTORNEWDAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxRqstOrNewDat`"]
pub struct TXRQSTORNEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQSTORNEWDAT_W<'a> {
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
#[doc = "Reader of field `ClrIntPnd`"]
pub type CLRINTPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ClrIntPnd`"]
pub struct CLRINTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINTPND_W<'a> {
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
#[doc = "Reader of field `Control`"]
pub type CONTROL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Control`"]
pub struct CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_W<'a> {
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
#[doc = "Reader of field `Arb`"]
pub type ARB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Arb`"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
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
#[doc = "Reader of field `Mask`"]
pub type MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Mask`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
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
#[doc = "Reader of field `WROrRD`"]
pub type WRORRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WROrRD`"]
pub struct WRORRD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRORRD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Access Data Bytes \\[7:4\\]
Direction = Write 1 = Transfer Data Bytes \\[7:4\\]
to Message Object. 0 = Data Bytes \\[7:4\\]
unchanged. Direction = Read 1 = Transfer Data Bytes \\[7:4\\]
to IF1 Message Buffer Register. 0 = Data Bytes \\[7:4\\]
unchanged."]
    #[inline(always)]
    pub fn dat_b(&self) -> DAT_B_R {
        DAT_B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes \\[3:0\\]
Direction = Write 1 = Transfer Data Bytes \\[3:0\\]
to Message Object 0 = Data Bytes \\[3:0\\]
unchanged. Direction = Read 1 = Transfer Data Bytes \\[3:0\\]
to IF1 Message Buffer Register. 0 = Data Bytes \\[3:0\\]
unchanged."]
    #[inline(always)]
    pub fn dat_a(&self) -> DAT_A_R {
        DAT_A_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Access Transmission Request Bit when Direction = Write 1 = Set TxRqst bit. 0 = TxRqst bit unchanged. Note: If a transmission is requested by programming bit TxRqst/NewDat in the IF1 Command Mask Register, bit TxRqst in the IF2 Message Control Register will be ignored. Access New Data Bit when Direction = Read 1 = Clear NewDat bit in the Message Object 0 = NewDat bit remains unchanged. Note : A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IF1 Message Control Register always reflect the status before resetting these bits."]
    #[inline(always)]
    pub fn tx_rqst_or_new_dat(&self) -> TXRQSTORNEWDAT_R {
        TXRQSTORNEWDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit Direction = Write When writing to a Message Object, this bit is ignored. Direction = Read 1 = Clear IntPnd bit in the Message Object. 0 = IntPnd bit remains unchanged."]
    #[inline(always)]
    pub fn clr_int_pnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control Access Control Bits Direction = Write 1 = Transfer Control Bits to Message Object. 0 = Control Bits unchanged Direction = Read 1 = Transfer Control Bits to IF1 Message Buffer Register. 0 = Control Bits unchanged."]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits Direction = Write 1 = Transfer Identifier + Dir + Xtd + MsgVal to Message Object 0 = Arbitration bits unchanged. Direction = Read 1 = Transfer Identifier + Dir + Xtd + MsgVal to IF1 Message Buffer Register. 0 = Arbitration bits unchanged."]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits Direction = Write 1 = Transfer Identifier Mask + MDir + MXtd to Message Object. 0: = Mask bits unchanged. Direction = Read 1 = Transfer Identifier Mask + MDir + MXtd to IF1 Message Buffer Register. 0 = Mask bits unchanged."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write / Read 1 = Write: Transfer data from the selected Message Buffer Registers to the Message Object addressed by the Command Request Register. 0 = Read: Transfer data from the Message Object addressed by the Command Request Register into the selected Message Buffer Registers."]
    #[inline(always)]
    pub fn wror_rd(&self) -> WRORRD_R {
        WRORRD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Data Bytes \\[7:4\\]
Direction = Write 1 = Transfer Data Bytes \\[7:4\\]
to Message Object. 0 = Data Bytes \\[7:4\\]
unchanged. Direction = Read 1 = Transfer Data Bytes \\[7:4\\]
to IF1 Message Buffer Register. 0 = Data Bytes \\[7:4\\]
unchanged."]
    #[inline(always)]
    pub fn dat_b(&mut self) -> DAT_B_W {
        DAT_B_W { w: self }
    }
    #[doc = "Bit 1 - Access Data Bytes \\[3:0\\]
Direction = Write 1 = Transfer Data Bytes \\[3:0\\]
to Message Object 0 = Data Bytes \\[3:0\\]
unchanged. Direction = Read 1 = Transfer Data Bytes \\[3:0\\]
to IF1 Message Buffer Register. 0 = Data Bytes \\[3:0\\]
unchanged."]
    #[inline(always)]
    pub fn dat_a(&mut self) -> DAT_A_W {
        DAT_A_W { w: self }
    }
    #[doc = "Bit 2 - Access Transmission Request Bit when Direction = Write 1 = Set TxRqst bit. 0 = TxRqst bit unchanged. Note: If a transmission is requested by programming bit TxRqst/NewDat in the IF1 Command Mask Register, bit TxRqst in the IF2 Message Control Register will be ignored. Access New Data Bit when Direction = Read 1 = Clear NewDat bit in the Message Object 0 = NewDat bit remains unchanged. Note : A read access to a Message Object can be combined with the reset of the control bits IntPnd and NewDat. The values of these bits transferred to the IF1 Message Control Register always reflect the status before resetting these bits."]
    #[inline(always)]
    pub fn tx_rqst_or_new_dat(&mut self) -> TXRQSTORNEWDAT_W {
        TXRQSTORNEWDAT_W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit Direction = Write When writing to a Message Object, this bit is ignored. Direction = Read 1 = Clear IntPnd bit in the Message Object. 0 = IntPnd bit remains unchanged."]
    #[inline(always)]
    pub fn clr_int_pnd(&mut self) -> CLRINTPND_W {
        CLRINTPND_W { w: self }
    }
    #[doc = "Bit 4 - Control Access Control Bits Direction = Write 1 = Transfer Control Bits to Message Object. 0 = Control Bits unchanged Direction = Read 1 = Transfer Control Bits to IF1 Message Buffer Register. 0 = Control Bits unchanged."]
    #[inline(always)]
    pub fn control(&mut self) -> CONTROL_W {
        CONTROL_W { w: self }
    }
    #[doc = "Bit 5 - Access Arbitration Bits Direction = Write 1 = Transfer Identifier + Dir + Xtd + MsgVal to Message Object 0 = Arbitration bits unchanged. Direction = Read 1 = Transfer Identifier + Dir + Xtd + MsgVal to IF1 Message Buffer Register. 0 = Arbitration bits unchanged."]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 6 - Access Mask Bits Direction = Write 1 = Transfer Identifier Mask + MDir + MXtd to Message Object. 0: = Mask bits unchanged. Direction = Read 1 = Transfer Identifier Mask + MDir + MXtd to IF1 Message Buffer Register. 0 = Mask bits unchanged."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 7 - Write / Read 1 = Write: Transfer data from the selected Message Buffer Registers to the Message Object addressed by the Command Request Register. 0 = Read: Transfer data from the Message Object addressed by the Command Request Register into the selected Message Buffer Registers."]
    #[inline(always)]
    pub fn wror_rd(&mut self) -> WRORRD_W {
        WRORRD_W { w: self }
    }
}
