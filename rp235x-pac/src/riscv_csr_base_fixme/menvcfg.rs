#[doc = "Register `MENVCFG` reader"]
pub type R = crate::R<MENVCFG_SPEC>;
#[doc = "Field `FIOM` reader - When set, fence instructions in modes less privileged than M-mode which specify that IO memory accesses are ordered will also cause ordering of main memory accesses.  

 FIOM is hardwired to zero on Hazard3, because S-mode is not supported, and because fence instructions execute as NOPs (with the exception of `fence.i`)"]
pub type FIOM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When set, fence instructions in modes less privileged than M-mode which specify that IO memory accesses are ordered will also cause ordering of main memory accesses.  

 FIOM is hardwired to zero on Hazard3, because S-mode is not supported, and because fence instructions execute as NOPs (with the exception of `fence.i`)"]
    #[inline(always)]
    pub fn fiom(&self) -> FIOM_R {
        FIOM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Machine environment configuration register, low half  

You can [`read`](crate::Reg::read) this register and get [`menvcfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MENVCFG_SPEC;
impl crate::RegisterSpec for MENVCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`menvcfg::R`](R) reader structure"]
impl crate::Readable for MENVCFG_SPEC {}
#[doc = "`reset()` method sets MENVCFG to value 0"]
impl crate::Resettable for MENVCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
