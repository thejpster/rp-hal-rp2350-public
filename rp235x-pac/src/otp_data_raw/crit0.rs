#[doc = "Register `CRIT0` reader"]
pub type R = crate::R<CRIT0_SPEC>;
#[doc = "Register `CRIT0` writer"]
pub type W = crate::W<CRIT0_SPEC>;
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
impl W {}
#[doc = "Page 0 critical boot flags (RBIT-8)  

You can [`read`](crate::Reg::read) this register and get [`crit0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crit0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT0_SPEC;
impl crate::RegisterSpec for CRIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit0::R`](R) reader structure"]
impl crate::Readable for CRIT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crit0::W`](W) writer structure"]
impl crate::Writable for CRIT0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRIT0 to value 0"]
impl crate::Resettable for CRIT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
