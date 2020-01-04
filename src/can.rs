#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub can_con: CAN_CON,
    #[doc = "0x04 - Status Register"]
    pub can_status: CAN_STATUS,
    #[doc = "0x08 - Error Counter"]
    pub can_err: CAN_ERR,
    #[doc = "0x0c - Bit Timing Register"]
    pub can_btime: CAN_BTIME,
    #[doc = "0x10 - Interrupt Identifier Register"]
    pub can_iidr: CAN_IIDR,
    #[doc = "0x14 - Test Register"]
    pub can_test: CAN_TEST,
    #[doc = "0x18 - BRP Extension Register"]
    pub can_brpe: CAN_BRPE,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - IF1 Command Request Register"]
    pub can_if1_creq: CAN_IF1_CREQ,
    #[doc = "0x24 - IF1 Command Mask Registers"]
    pub can_if1_cmask: CAN_IF1_CMASK,
    #[doc = "0x28 - IF1 Mask 1 Register"]
    pub can_if1_mask1: CAN_IF1_MASK1,
    #[doc = "0x2c - IF1 Mask 2 Register"]
    pub can_if1_mask2: CAN_IF1_MASK2,
    #[doc = "0x30 - IF1 Arbitration 1 Register"]
    pub can_if1_arb1: CAN_IF1_ARB1,
    #[doc = "0x34 - IF1 Arbitration 2 Register"]
    pub can_if1_arb2: CAN_IF1_ARB2,
    #[doc = "0x38 - IF1 Message Control Registers"]
    pub can_if1_mcon: CAN_IF1_MCON,
    #[doc = "0x3c - IF1 Data A1 Registers"]
    pub can_if1_dat_a1: CAN_IF1_DAT_A1,
    #[doc = "0x40 - IF1 Data A2 Registers"]
    pub can_if1_dat_a2: CAN_IF1_DAT_A2,
    #[doc = "0x44 - IF1 Data B1 Registers"]
    pub can_if1_dat_b1: CAN_IF1_DAT_B1,
    #[doc = "0x48 - IF1 Data B2 Registers"]
    pub can_if1_dat_b2: CAN_IF1_DAT_B2,
    _reserved18: [u8; 52usize],
    #[doc = "0x80 - IF2 Command Request Register"]
    pub can_if2_creq: CAN_IF2_CREQ,
    #[doc = "0x84 - IF2 Command Mask Register"]
    pub can_if2_cmask: CAN_IF2_CMASK,
    #[doc = "0x88 - IF2 Mask 1 Registers"]
    pub can_if2_mask1: CAN_IF2_MASK1,
    #[doc = "0x8c - IF2 Mask 2 Registers"]
    pub can_if2_mask2: CAN_IF2_MASK2,
    #[doc = "0x90 - IF2 Arbitration 1 Register"]
    pub can_if2_arb1: CAN_IF2_ARB1,
    #[doc = "0x94 - IF2 Arbitration 2 Register"]
    pub can_if2_arb2: CAN_IF2_ARB2,
    #[doc = "0x98 - IF2 Message Control Register"]
    pub can_if2_mcon: CAN_IF2_MCON,
    #[doc = "0x9c - IF2 Data A1 Registers"]
    pub can_if2_dat_a1: CAN_IF2_DAT_A1,
    #[doc = "0xa0 - IF2 Data A2 Registers"]
    pub can_if2_dat_a2: CAN_IF2_DAT_A2,
    #[doc = "0xa4 - IF2 Data B1 Registers"]
    pub can_if2_dat_b1: CAN_IF2_DAT_B1,
    #[doc = "0xa8 - IF2 Data B2 Registers"]
    pub can_if2_dat_b2: CAN_IF2_DAT_B2,
    _reserved29: [u8; 84usize],
    #[doc = "0x100 - Transmission Request Registers 1"]
    pub can_txreq1: CAN_TXREQ1,
    #[doc = "0x104 - Transmission Request Register 2"]
    pub can_txreq2: CAN_TXREQ2,
    _reserved31: [u8; 24usize],
    #[doc = "0x120 - New Data Register 1"]
    pub can_ndat1: CAN_NDAT1,
    #[doc = "0x124 - New Data Register 2"]
    pub can_ndat2: CAN_NDAT2,
    _reserved33: [u8; 24usize],
    #[doc = "0x140 - Interrupt Pending Register 1"]
    pub can_ipnd1: CAN_IPND1,
    #[doc = "0x144 - Interrupt Pending Register 2"]
    pub can_ipnd2: CAN_IPND2,
    _reserved35: [u8; 24usize],
    #[doc = "0x160 - Message Valid Register 1"]
    pub can_mvld1: CAN_MVLD1,
    #[doc = "0x164 - Message Valid Register 2"]
    pub can_mvld2: CAN_MVLD2,
    #[doc = "0x168 - Wake Up Function Enable"]
    pub can_wu_en: CAN_WU_EN,
    #[doc = "0x16c - Wake Up Function Status"]
    pub can_wu_status: CAN_WU_STATUS,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_con](can_con) module"]
pub type CAN_CON = crate::Reg<u32, _CAN_CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_CON;
#[doc = "`read()` method returns [can_con::R](can_con::R) reader structure"]
impl crate::Readable for CAN_CON {}
#[doc = "`write(|w| ..)` method takes [can_con::W](can_con::W) writer structure"]
impl crate::Writable for CAN_CON {}
#[doc = "Control Register"]
pub mod can_con;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_status](can_status) module"]
pub type CAN_STATUS = crate::Reg<u32, _CAN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_STATUS;
#[doc = "`read()` method returns [can_status::R](can_status::R) reader structure"]
impl crate::Readable for CAN_STATUS {}
#[doc = "`write(|w| ..)` method takes [can_status::W](can_status::W) writer structure"]
impl crate::Writable for CAN_STATUS {}
#[doc = "Status Register"]
pub mod can_status;
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_err](can_err) module"]
pub type CAN_ERR = crate::Reg<u32, _CAN_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_ERR;
#[doc = "`read()` method returns [can_err::R](can_err::R) reader structure"]
impl crate::Readable for CAN_ERR {}
#[doc = "Error Counter"]
pub mod can_err;
#[doc = "Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_btime](can_btime) module"]
pub type CAN_BTIME = crate::Reg<u32, _CAN_BTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_BTIME;
#[doc = "`read()` method returns [can_btime::R](can_btime::R) reader structure"]
impl crate::Readable for CAN_BTIME {}
#[doc = "`write(|w| ..)` method takes [can_btime::W](can_btime::W) writer structure"]
impl crate::Writable for CAN_BTIME {}
#[doc = "Bit Timing Register"]
pub mod can_btime;
#[doc = "Interrupt Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_iidr](can_iidr) module"]
pub type CAN_IIDR = crate::Reg<u32, _CAN_IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IIDR;
#[doc = "`read()` method returns [can_iidr::R](can_iidr::R) reader structure"]
impl crate::Readable for CAN_IIDR {}
#[doc = "Interrupt Identifier Register"]
pub mod can_iidr;
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_test](can_test) module"]
pub type CAN_TEST = crate::Reg<u32, _CAN_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TEST;
#[doc = "`read()` method returns [can_test::R](can_test::R) reader structure"]
impl crate::Readable for CAN_TEST {}
#[doc = "`write(|w| ..)` method takes [can_test::W](can_test::W) writer structure"]
impl crate::Writable for CAN_TEST {}
#[doc = "Test Register"]
pub mod can_test;
#[doc = "BRP Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_brpe](can_brpe) module"]
pub type CAN_BRPE = crate::Reg<u32, _CAN_BRPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_BRPE;
#[doc = "`read()` method returns [can_brpe::R](can_brpe::R) reader structure"]
impl crate::Readable for CAN_BRPE {}
#[doc = "`write(|w| ..)` method takes [can_brpe::W](can_brpe::W) writer structure"]
impl crate::Writable for CAN_BRPE {}
#[doc = "BRP Extension Register"]
pub mod can_brpe;
#[doc = "IF1 Command Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_creq](can_if1_creq) module"]
pub type CAN_IF1_CREQ = crate::Reg<u32, _CAN_IF1_CREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_CREQ;
#[doc = "`read()` method returns [can_if1_creq::R](can_if1_creq::R) reader structure"]
impl crate::Readable for CAN_IF1_CREQ {}
#[doc = "`write(|w| ..)` method takes [can_if1_creq::W](can_if1_creq::W) writer structure"]
impl crate::Writable for CAN_IF1_CREQ {}
#[doc = "IF1 Command Request Register"]
pub mod can_if1_creq;
#[doc = "IF1 Command Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_cmask](can_if1_cmask) module"]
pub type CAN_IF1_CMASK = crate::Reg<u32, _CAN_IF1_CMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_CMASK;
#[doc = "`read()` method returns [can_if1_cmask::R](can_if1_cmask::R) reader structure"]
impl crate::Readable for CAN_IF1_CMASK {}
#[doc = "`write(|w| ..)` method takes [can_if1_cmask::W](can_if1_cmask::W) writer structure"]
impl crate::Writable for CAN_IF1_CMASK {}
#[doc = "IF1 Command Mask Registers"]
pub mod can_if1_cmask;
#[doc = "IF1 Mask 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_mask1](can_if1_mask1) module"]
pub type CAN_IF1_MASK1 = crate::Reg<u32, _CAN_IF1_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_MASK1;
#[doc = "`read()` method returns [can_if1_mask1::R](can_if1_mask1::R) reader structure"]
impl crate::Readable for CAN_IF1_MASK1 {}
#[doc = "`write(|w| ..)` method takes [can_if1_mask1::W](can_if1_mask1::W) writer structure"]
impl crate::Writable for CAN_IF1_MASK1 {}
#[doc = "IF1 Mask 1 Register"]
pub mod can_if1_mask1;
#[doc = "IF1 Mask 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_mask2](can_if1_mask2) module"]
pub type CAN_IF1_MASK2 = crate::Reg<u32, _CAN_IF1_MASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_MASK2;
#[doc = "`read()` method returns [can_if1_mask2::R](can_if1_mask2::R) reader structure"]
impl crate::Readable for CAN_IF1_MASK2 {}
#[doc = "`write(|w| ..)` method takes [can_if1_mask2::W](can_if1_mask2::W) writer structure"]
impl crate::Writable for CAN_IF1_MASK2 {}
#[doc = "IF1 Mask 2 Register"]
pub mod can_if1_mask2;
#[doc = "IF1 Arbitration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_arb1](can_if1_arb1) module"]
pub type CAN_IF1_ARB1 = crate::Reg<u32, _CAN_IF1_ARB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_ARB1;
#[doc = "`read()` method returns [can_if1_arb1::R](can_if1_arb1::R) reader structure"]
impl crate::Readable for CAN_IF1_ARB1 {}
#[doc = "`write(|w| ..)` method takes [can_if1_arb1::W](can_if1_arb1::W) writer structure"]
impl crate::Writable for CAN_IF1_ARB1 {}
#[doc = "IF1 Arbitration 1 Register"]
pub mod can_if1_arb1;
#[doc = "IF1 Arbitration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_arb2](can_if1_arb2) module"]
pub type CAN_IF1_ARB2 = crate::Reg<u32, _CAN_IF1_ARB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_ARB2;
#[doc = "`read()` method returns [can_if1_arb2::R](can_if1_arb2::R) reader structure"]
impl crate::Readable for CAN_IF1_ARB2 {}
#[doc = "`write(|w| ..)` method takes [can_if1_arb2::W](can_if1_arb2::W) writer structure"]
impl crate::Writable for CAN_IF1_ARB2 {}
#[doc = "IF1 Arbitration 2 Register"]
pub mod can_if1_arb2;
#[doc = "IF1 Message Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_mcon](can_if1_mcon) module"]
pub type CAN_IF1_MCON = crate::Reg<u32, _CAN_IF1_MCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_MCON;
#[doc = "`read()` method returns [can_if1_mcon::R](can_if1_mcon::R) reader structure"]
impl crate::Readable for CAN_IF1_MCON {}
#[doc = "`write(|w| ..)` method takes [can_if1_mcon::W](can_if1_mcon::W) writer structure"]
impl crate::Writable for CAN_IF1_MCON {}
#[doc = "IF1 Message Control Registers"]
pub mod can_if1_mcon;
#[doc = "IF1 Data A1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_dat_a1](can_if1_dat_a1) module"]
pub type CAN_IF1_DAT_A1 = crate::Reg<u32, _CAN_IF1_DAT_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_DAT_A1;
#[doc = "`read()` method returns [can_if1_dat_a1::R](can_if1_dat_a1::R) reader structure"]
impl crate::Readable for CAN_IF1_DAT_A1 {}
#[doc = "`write(|w| ..)` method takes [can_if1_dat_a1::W](can_if1_dat_a1::W) writer structure"]
impl crate::Writable for CAN_IF1_DAT_A1 {}
#[doc = "IF1 Data A1 Registers"]
pub mod can_if1_dat_a1;
#[doc = "IF1 Data A2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_dat_a2](can_if1_dat_a2) module"]
pub type CAN_IF1_DAT_A2 = crate::Reg<u32, _CAN_IF1_DAT_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_DAT_A2;
#[doc = "`read()` method returns [can_if1_dat_a2::R](can_if1_dat_a2::R) reader structure"]
impl crate::Readable for CAN_IF1_DAT_A2 {}
#[doc = "`write(|w| ..)` method takes [can_if1_dat_a2::W](can_if1_dat_a2::W) writer structure"]
impl crate::Writable for CAN_IF1_DAT_A2 {}
#[doc = "IF1 Data A2 Registers"]
pub mod can_if1_dat_a2;
#[doc = "IF1 Data B1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_dat_b1](can_if1_dat_b1) module"]
pub type CAN_IF1_DAT_B1 = crate::Reg<u32, _CAN_IF1_DAT_B1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_DAT_B1;
#[doc = "`read()` method returns [can_if1_dat_b1::R](can_if1_dat_b1::R) reader structure"]
impl crate::Readable for CAN_IF1_DAT_B1 {}
#[doc = "`write(|w| ..)` method takes [can_if1_dat_b1::W](can_if1_dat_b1::W) writer structure"]
impl crate::Writable for CAN_IF1_DAT_B1 {}
#[doc = "IF1 Data B1 Registers"]
pub mod can_if1_dat_b1;
#[doc = "IF1 Data B2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if1_dat_b2](can_if1_dat_b2) module"]
pub type CAN_IF1_DAT_B2 = crate::Reg<u32, _CAN_IF1_DAT_B2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF1_DAT_B2;
#[doc = "`read()` method returns [can_if1_dat_b2::R](can_if1_dat_b2::R) reader structure"]
impl crate::Readable for CAN_IF1_DAT_B2 {}
#[doc = "`write(|w| ..)` method takes [can_if1_dat_b2::W](can_if1_dat_b2::W) writer structure"]
impl crate::Writable for CAN_IF1_DAT_B2 {}
#[doc = "IF1 Data B2 Registers"]
pub mod can_if1_dat_b2;
#[doc = "IF2 Command Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_creq](can_if2_creq) module"]
pub type CAN_IF2_CREQ = crate::Reg<u32, _CAN_IF2_CREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_CREQ;
#[doc = "`read()` method returns [can_if2_creq::R](can_if2_creq::R) reader structure"]
impl crate::Readable for CAN_IF2_CREQ {}
#[doc = "`write(|w| ..)` method takes [can_if2_creq::W](can_if2_creq::W) writer structure"]
impl crate::Writable for CAN_IF2_CREQ {}
#[doc = "IF2 Command Request Register"]
pub mod can_if2_creq;
#[doc = "IF2 Command Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_cmask](can_if2_cmask) module"]
pub type CAN_IF2_CMASK = crate::Reg<u32, _CAN_IF2_CMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_CMASK;
#[doc = "`read()` method returns [can_if2_cmask::R](can_if2_cmask::R) reader structure"]
impl crate::Readable for CAN_IF2_CMASK {}
#[doc = "`write(|w| ..)` method takes [can_if2_cmask::W](can_if2_cmask::W) writer structure"]
impl crate::Writable for CAN_IF2_CMASK {}
#[doc = "IF2 Command Mask Register"]
pub mod can_if2_cmask;
#[doc = "IF2 Mask 1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mask1](can_if2_mask1) module"]
pub type CAN_IF2_MASK1 = crate::Reg<u32, _CAN_IF2_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_MASK1;
#[doc = "`read()` method returns [can_if2_mask1::R](can_if2_mask1::R) reader structure"]
impl crate::Readable for CAN_IF2_MASK1 {}
#[doc = "`write(|w| ..)` method takes [can_if2_mask1::W](can_if2_mask1::W) writer structure"]
impl crate::Writable for CAN_IF2_MASK1 {}
#[doc = "IF2 Mask 1 Registers"]
pub mod can_if2_mask1;
#[doc = "IF2 Mask 2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mask2](can_if2_mask2) module"]
pub type CAN_IF2_MASK2 = crate::Reg<u32, _CAN_IF2_MASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_MASK2;
#[doc = "`read()` method returns [can_if2_mask2::R](can_if2_mask2::R) reader structure"]
impl crate::Readable for CAN_IF2_MASK2 {}
#[doc = "`write(|w| ..)` method takes [can_if2_mask2::W](can_if2_mask2::W) writer structure"]
impl crate::Writable for CAN_IF2_MASK2 {}
#[doc = "IF2 Mask 2 Registers"]
pub mod can_if2_mask2;
#[doc = "IF2 Arbitration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_arb1](can_if2_arb1) module"]
pub type CAN_IF2_ARB1 = crate::Reg<u32, _CAN_IF2_ARB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_ARB1;
#[doc = "`read()` method returns [can_if2_arb1::R](can_if2_arb1::R) reader structure"]
impl crate::Readable for CAN_IF2_ARB1 {}
#[doc = "`write(|w| ..)` method takes [can_if2_arb1::W](can_if2_arb1::W) writer structure"]
impl crate::Writable for CAN_IF2_ARB1 {}
#[doc = "IF2 Arbitration 1 Register"]
pub mod can_if2_arb1;
#[doc = "IF2 Arbitration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_arb2](can_if2_arb2) module"]
pub type CAN_IF2_ARB2 = crate::Reg<u32, _CAN_IF2_ARB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_ARB2;
#[doc = "`read()` method returns [can_if2_arb2::R](can_if2_arb2::R) reader structure"]
impl crate::Readable for CAN_IF2_ARB2 {}
#[doc = "`write(|w| ..)` method takes [can_if2_arb2::W](can_if2_arb2::W) writer structure"]
impl crate::Writable for CAN_IF2_ARB2 {}
#[doc = "IF2 Arbitration 2 Register"]
pub mod can_if2_arb2;
#[doc = "IF2 Message Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mcon](can_if2_mcon) module"]
pub type CAN_IF2_MCON = crate::Reg<u32, _CAN_IF2_MCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_MCON;
#[doc = "`read()` method returns [can_if2_mcon::R](can_if2_mcon::R) reader structure"]
impl crate::Readable for CAN_IF2_MCON {}
#[doc = "`write(|w| ..)` method takes [can_if2_mcon::W](can_if2_mcon::W) writer structure"]
impl crate::Writable for CAN_IF2_MCON {}
#[doc = "IF2 Message Control Register"]
pub mod can_if2_mcon;
#[doc = "IF2 Data A1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_dat_a1](can_if2_dat_a1) module"]
pub type CAN_IF2_DAT_A1 = crate::Reg<u32, _CAN_IF2_DAT_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_DAT_A1;
#[doc = "`read()` method returns [can_if2_dat_a1::R](can_if2_dat_a1::R) reader structure"]
impl crate::Readable for CAN_IF2_DAT_A1 {}
#[doc = "`write(|w| ..)` method takes [can_if2_dat_a1::W](can_if2_dat_a1::W) writer structure"]
impl crate::Writable for CAN_IF2_DAT_A1 {}
#[doc = "IF2 Data A1 Registers"]
pub mod can_if2_dat_a1;
#[doc = "IF2 Data A2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_dat_a2](can_if2_dat_a2) module"]
pub type CAN_IF2_DAT_A2 = crate::Reg<u32, _CAN_IF2_DAT_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_DAT_A2;
#[doc = "`read()` method returns [can_if2_dat_a2::R](can_if2_dat_a2::R) reader structure"]
impl crate::Readable for CAN_IF2_DAT_A2 {}
#[doc = "`write(|w| ..)` method takes [can_if2_dat_a2::W](can_if2_dat_a2::W) writer structure"]
impl crate::Writable for CAN_IF2_DAT_A2 {}
#[doc = "IF2 Data A2 Registers"]
pub mod can_if2_dat_a2;
#[doc = "IF2 Data B1 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_dat_b1](can_if2_dat_b1) module"]
pub type CAN_IF2_DAT_B1 = crate::Reg<u32, _CAN_IF2_DAT_B1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_DAT_B1;
#[doc = "`read()` method returns [can_if2_dat_b1::R](can_if2_dat_b1::R) reader structure"]
impl crate::Readable for CAN_IF2_DAT_B1 {}
#[doc = "`write(|w| ..)` method takes [can_if2_dat_b1::W](can_if2_dat_b1::W) writer structure"]
impl crate::Writable for CAN_IF2_DAT_B1 {}
#[doc = "IF2 Data B1 Registers"]
pub mod can_if2_dat_b1;
#[doc = "IF2 Data B2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_dat_b2](can_if2_dat_b2) module"]
pub type CAN_IF2_DAT_B2 = crate::Reg<u32, _CAN_IF2_DAT_B2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IF2_DAT_B2;
#[doc = "`read()` method returns [can_if2_dat_b2::R](can_if2_dat_b2::R) reader structure"]
impl crate::Readable for CAN_IF2_DAT_B2 {}
#[doc = "`write(|w| ..)` method takes [can_if2_dat_b2::W](can_if2_dat_b2::W) writer structure"]
impl crate::Writable for CAN_IF2_DAT_B2 {}
#[doc = "IF2 Data B2 Registers"]
pub mod can_if2_dat_b2;
#[doc = "Transmission Request Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_txreq1](can_txreq1) module"]
pub type CAN_TXREQ1 = crate::Reg<u32, _CAN_TXREQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TXREQ1;
#[doc = "`read()` method returns [can_txreq1::R](can_txreq1::R) reader structure"]
impl crate::Readable for CAN_TXREQ1 {}
#[doc = "Transmission Request Registers 1"]
pub mod can_txreq1;
#[doc = "Transmission Request Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_txreq2](can_txreq2) module"]
pub type CAN_TXREQ2 = crate::Reg<u32, _CAN_TXREQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TXREQ2;
#[doc = "`read()` method returns [can_txreq2::R](can_txreq2::R) reader structure"]
impl crate::Readable for CAN_TXREQ2 {}
#[doc = "Transmission Request Register 2"]
pub mod can_txreq2;
#[doc = "New Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ndat1](can_ndat1) module"]
pub type CAN_NDAT1 = crate::Reg<u32, _CAN_NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_NDAT1;
#[doc = "`read()` method returns [can_ndat1::R](can_ndat1::R) reader structure"]
impl crate::Readable for CAN_NDAT1 {}
#[doc = "New Data Register 1"]
pub mod can_ndat1;
#[doc = "New Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ndat2](can_ndat2) module"]
pub type CAN_NDAT2 = crate::Reg<u32, _CAN_NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_NDAT2;
#[doc = "`read()` method returns [can_ndat2::R](can_ndat2::R) reader structure"]
impl crate::Readable for CAN_NDAT2 {}
#[doc = "New Data Register 2"]
pub mod can_ndat2;
#[doc = "Interrupt Pending Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ipnd1](can_ipnd1) module"]
pub type CAN_IPND1 = crate::Reg<u32, _CAN_IPND1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IPND1;
#[doc = "`read()` method returns [can_ipnd1::R](can_ipnd1::R) reader structure"]
impl crate::Readable for CAN_IPND1 {}
#[doc = "Interrupt Pending Register 1"]
pub mod can_ipnd1;
#[doc = "Interrupt Pending Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ipnd2](can_ipnd2) module"]
pub type CAN_IPND2 = crate::Reg<u32, _CAN_IPND2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IPND2;
#[doc = "`read()` method returns [can_ipnd2::R](can_ipnd2::R) reader structure"]
impl crate::Readable for CAN_IPND2 {}
#[doc = "Interrupt Pending Register 2"]
pub mod can_ipnd2;
#[doc = "Message Valid Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_mvld1](can_mvld1) module"]
pub type CAN_MVLD1 = crate::Reg<u32, _CAN_MVLD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_MVLD1;
#[doc = "`read()` method returns [can_mvld1::R](can_mvld1::R) reader structure"]
impl crate::Readable for CAN_MVLD1 {}
#[doc = "Message Valid Register 1"]
pub mod can_mvld1;
#[doc = "Message Valid Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_mvld2](can_mvld2) module"]
pub type CAN_MVLD2 = crate::Reg<u32, _CAN_MVLD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_MVLD2;
#[doc = "`read()` method returns [can_mvld2::R](can_mvld2::R) reader structure"]
impl crate::Readable for CAN_MVLD2 {}
#[doc = "Message Valid Register 2"]
pub mod can_mvld2;
#[doc = "Wake Up Function Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_wu_en](can_wu_en) module"]
pub type CAN_WU_EN = crate::Reg<u32, _CAN_WU_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_WU_EN;
#[doc = "`read()` method returns [can_wu_en::R](can_wu_en::R) reader structure"]
impl crate::Readable for CAN_WU_EN {}
#[doc = "`write(|w| ..)` method takes [can_wu_en::W](can_wu_en::W) writer structure"]
impl crate::Writable for CAN_WU_EN {}
#[doc = "Wake Up Function Enable"]
pub mod can_wu_en;
#[doc = "Wake Up Function Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_wu_status](can_wu_status) module"]
pub type CAN_WU_STATUS = crate::Reg<u32, _CAN_WU_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_WU_STATUS;
#[doc = "`read()` method returns [can_wu_status::R](can_wu_status::R) reader structure"]
impl crate::Readable for CAN_WU_STATUS {}
#[doc = "`write(|w| ..)` method takes [can_wu_status::W](can_wu_status::W) writer structure"]
impl crate::Writable for CAN_WU_STATUS {}
#[doc = "Wake Up Function Status"]
pub mod can_wu_status;
