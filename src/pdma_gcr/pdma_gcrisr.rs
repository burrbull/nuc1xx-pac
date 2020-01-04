#[doc = "Reader of register PDMA_GCRISR"]
pub type R = crate::R<u32, super::PDMA_GCRISR>;
#[doc = "Reader of field `INTR0`"]
pub type INTR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR1`"]
pub type INTR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR2`"]
pub type INTR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR3`"]
pub type INTR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR4`"]
pub type INTR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR5`"]
pub type INTR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR6`"]
pub type INTR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR7`"]
pub type INTR7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR8`"]
pub type INTR8_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTR`"]
pub type INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt Pin Status of Channel 0 This bit is the Interrupt pin status of PDMA channel0. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr0(&self) -> INTR0_R {
        INTR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Pin Status of Channel 1 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel1. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr1(&self) -> INTR1_R {
        INTR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Pin Status of Channel 2 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel2. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr2(&self) -> INTR2_R {
        INTR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Pin Status of Channel 3 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel3. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr3(&self) -> INTR3_R {
        INTR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Pin Status of Channel 4 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel4. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr4(&self) -> INTR4_R {
        INTR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Pin Status of Channel 5 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel5. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr5(&self) -> INTR5_R {
        INTR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Pin Status of Channel 6 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel 6. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr6(&self) -> INTR6_R {
        INTR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Pin Status of Channel 7 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel 7. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr7(&self) -> INTR7_R {
        INTR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt Pin Status of Channel 4 (Medium Density Only) This bit is the Interrupt pin status of PDMA channel 8. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr8(&self) -> INTR8_R {
        INTR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt Pin Status This bit is the Interrupt pin status of PDMA controller. Note: This bit is read only"]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
