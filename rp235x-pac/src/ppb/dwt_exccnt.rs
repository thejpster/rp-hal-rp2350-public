#[doc = "Register `DWT_EXCCNT` reader"]
pub type R = crate::R<DWT_EXCCNT_SPEC>;
#[doc = "Register `DWT_EXCCNT` writer"]
pub type W = crate::W<DWT_EXCCNT_SPEC>;
#[doc = "Field `EXCCNT` reader - Counts one on each cycle when all of the following are true: - DWT_CTRL.EXCEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - An exception-entry or exception-exit related operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
pub type EXCCNT_R = crate::FieldReader;
#[doc = "Field `EXCCNT` writer - Counts one on each cycle when all of the following are true: - DWT_CTRL.EXCEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - An exception-entry or exception-exit related operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
pub type EXCCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counts one on each cycle when all of the following are true: - DWT_CTRL.EXCEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - An exception-entry or exception-exit related operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    pub fn exccnt(&self) -> EXCCNT_R {
        EXCCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counts one on each cycle when all of the following are true: - DWT_CTRL.EXCEVTENA == 1 and DEMCR.TRCENA == 1. - No instruction is executed, see DWT_CPICNT. - An exception-entry or exception-exit related operation is in progress. - Either SecureNoninvasiveDebugAllowed() == TRUE, or NS-Req for the operation is set to Non-secure and NoninvasiveDebugAllowed() == TRUE."]
    #[inline(always)]
    #[must_use]
    pub fn exccnt(&mut self) -> EXCCNT_W<DWT_EXCCNT_SPEC> {
        EXCCNT_W::new(self, 0)
    }
}
#[doc = "Counts the total cycles spent in exception processing  

You can [`read`](crate::Reg::read) this register and get [`dwt_exccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_exccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_EXCCNT_SPEC;
impl crate::RegisterSpec for DWT_EXCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_exccnt::R`](R) reader structure"]
impl crate::Readable for DWT_EXCCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_exccnt::W`](W) writer structure"]
impl crate::Writable for DWT_EXCCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_EXCCNT to value 0"]
impl crate::Resettable for DWT_EXCCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
