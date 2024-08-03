#[doc = "Register `PROC_CONFIG` reader"]
pub type R = crate::R<PROC_CONFIG_SPEC>;
#[doc = "Field `PROC0_HALTED` reader - Indication that proc0 has halted"]
pub type PROC0_HALTED_R = crate::BitReader;
#[doc = "Field `PROC1_HALTED` reader - Indication that proc1 has halted"]
pub type PROC1_HALTED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indication that proc0 has halted"]
    #[inline(always)]
    pub fn proc0_halted(&self) -> PROC0_HALTED_R {
        PROC0_HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indication that proc1 has halted"]
    #[inline(always)]
    pub fn proc1_halted(&self) -> PROC1_HALTED_R {
        PROC1_HALTED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Configuration for processors  

You can [`read`](crate::Reg::read) this register and get [`proc_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_CONFIG_SPEC;
impl crate::RegisterSpec for PROC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_config::R`](R) reader structure"]
impl crate::Readable for PROC_CONFIG_SPEC {}
#[doc = "`reset()` method sets PROC_CONFIG to value 0"]
impl crate::Resettable for PROC_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
