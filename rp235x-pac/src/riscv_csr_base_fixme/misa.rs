#[doc = "Register `MISA` reader"]
pub type R = crate::R<MISA_SPEC>;
#[doc = "Field `A` reader - Value of 1 indicates the A extension (atomics) is implemented."]
pub type A_R = crate::BitReader;
#[doc = "Field `C` reader - Value of 1 indicates the C extension (compressed instructions) is implemented."]
pub type C_R = crate::BitReader;
#[doc = "Field `M` reader - Value of 1 indicates the M extension (integer multiply/divide) is implemented."]
pub type M_R = crate::BitReader;
#[doc = "Field `U` reader - Value of 1 indicates U-mode is implemented."]
pub type U_R = crate::BitReader;
#[doc = "Field `X` reader - Value of 1 indicates nonstandard extensions are present. (Xh3b bit manipulation, and custom sleep and interrupt control CSRs)"]
pub type X_R = crate::BitReader;
#[doc = "Field `MXL` reader - Value of 0x1 indicates this is a 32-bit processor."]
pub type MXL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Value of 1 indicates the A extension (atomics) is implemented."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Value of 1 indicates the C extension (compressed instructions) is implemented."]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - Value of 1 indicates the M extension (integer multiply/divide) is implemented."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20 - Value of 1 indicates U-mode is implemented."]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Value of 1 indicates nonstandard extensions are present. (Xh3b bit manipulation, and custom sleep and interrupt control CSRs)"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Value of 0x1 indicates this is a 32-bit processor."]
    #[inline(always)]
    pub fn mxl(&self) -> MXL_R {
        MXL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Summary of ISA extension support  

 On RP2350, Hazard3's full `-march` string is: `rv32ima_zicsr_zifencei_zba_zbb_zbs_zbkb_zca_zcb_zcmp`  

 Note Zca is equivalent to the C extension in this case; all instructions from the RISC-V C extension relevant to a 32-bit non-floating-point processor are supported. On older toolchains which do not support the Zc extensions, the appropriate `-march` string is: `rv32imac_zicsr_zifencei_zba_zbb_zbs_zbkb`  

 In addition the following custom extensions are configured: Xh3bm, Xh3power, Xh3irq, Xh3pmpm  

You can [`read`](crate::Reg::read) this register and get [`misa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISA_SPEC;
impl crate::RegisterSpec for MISA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misa::R`](R) reader structure"]
impl crate::Readable for MISA_SPEC {}
#[doc = "`reset()` method sets MISA to value 0x4090_1005"]
impl crate::Resettable for MISA_SPEC {
    const RESET_VALUE: u32 = 0x4090_1005;
}
