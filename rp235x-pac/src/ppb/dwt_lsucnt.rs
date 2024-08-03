#[doc = "Register `DWT_LSUCNT` reader"]
pub type R = crate::R<DWT_LSUCNT_SPEC>;
#[doc = "Register `DWT_LSUCNT` writer"]
pub type W = crate::W<DWT_LSUCNT_SPEC>;
#[doc = "Field `LSUCNT` reader - Counts one on each cycle when all of the following are true: - DWT_CTRL.LSUEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - A load-store operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
pub type LSUCNT_R = crate::FieldReader;
#[doc = "Field `LSUCNT` writer - Counts one on each cycle when all of the following are true: - DWT_CTRL.LSUEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - A load-store operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
pub type LSUCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counts one on each cycle when all of the following are true: - DWT_CTRL.LSUEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - A load-store operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    pub fn lsucnt(&self) -> LSUCNT_R {
        LSUCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counts one on each cycle when all of the following are true: - DWT_CTRL.LSUEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - No exception-entry or exception-exit operation is in progress, see DWT_EXCCNT. - A load-store operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    #[must_use]
    pub fn lsucnt(&mut self) -> LSUCNT_W<DWT_LSUCNT_SPEC> {
        LSUCNT_W::new(self, 0)
    }
}
#[doc = "Increments on the additional cycles required to execute all load or store instructions  

You can [`read`](crate::Reg::read) this register and get [`dwt_lsucnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_lsucnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_LSUCNT_SPEC;
impl crate::RegisterSpec for DWT_LSUCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_lsucnt::R`](R) reader structure"]
impl crate::Readable for DWT_LSUCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_lsucnt::W`](W) writer structure"]
impl crate::Writable for DWT_LSUCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_LSUCNT to value 0"]
impl crate::Resettable for DWT_LSUCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
