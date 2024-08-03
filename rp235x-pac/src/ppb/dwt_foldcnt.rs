#[doc = "Register `DWT_FOLDCNT` reader"]
pub type R = crate::R<DWT_FOLDCNT_SPEC>;
#[doc = "Register `DWT_FOLDCNT` writer"]
pub type W = crate::W<DWT_FOLDCNT_SPEC>;
#[doc = "Field `FOLDCNT` reader - Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FOLDCNT_R = crate::FieldReader;
#[doc = "Field `FOLDCNT` writer - Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FOLDCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    pub fn foldcnt(&self) -> FOLDCNT_R {
        FOLDCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    #[must_use]
    pub fn foldcnt(&mut self) -> FOLDCNT_W<DWT_FOLDCNT_SPEC> {
        FOLDCNT_W::new(self, 0)
    }
}
#[doc = "Increments on the additional cycles required to execute all load or store instructions  

You can [`read`](crate::Reg::read) this register and get [`dwt_foldcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_foldcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_FOLDCNT_SPEC;
impl crate::RegisterSpec for DWT_FOLDCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_foldcnt::R`](R) reader structure"]
impl crate::Readable for DWT_FOLDCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_foldcnt::W`](W) writer structure"]
impl crate::Writable for DWT_FOLDCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_FOLDCNT to value 0"]
impl crate::Resettable for DWT_FOLDCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
