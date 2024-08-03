#[doc = "Register `CRIT0` reader"]
pub type R = crate::R<CRIT0_SPEC>;
#[doc = "Field `ARM_DISABLE` reader - Permanently disable ARM processors (Cortex-M33)"]
pub type ARM_DISABLE_R = crate::BitReader;
#[doc = "Field `RISCV_DISABLE` reader - Permanently disable RISC-V processors (Hazard3)"]
pub type RISCV_DISABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Permanently disable ARM processors (Cortex-M33)"]
    #[inline(always)]
    pub fn arm_disable(&self) -> ARM_DISABLE_R {
        ARM_DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Permanently disable RISC-V processors (Hazard3)"]
    #[inline(always)]
    pub fn riscv_disable(&self) -> RISCV_DISABLE_R {
        RISCV_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Page 0 critical boot flags (RBIT-8)  

You can [`read`](crate::Reg::read) this register and get [`crit0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT0_SPEC;
impl crate::RegisterSpec for CRIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit0::R`](R) reader structure"]
impl crate::Readable for CRIT0_SPEC {}
#[doc = "`reset()` method sets CRIT0 to value 0"]
impl crate::Resettable for CRIT0_SPEC {
    const RESET_VALUE: u32 = 0;
}