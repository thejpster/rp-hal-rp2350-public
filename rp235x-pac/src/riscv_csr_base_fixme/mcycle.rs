#[doc = "Register `MCYCLE` reader"]
pub type R = crate::R<MCYCLE_SPEC>;
#[doc = "Register `MCYCLE` writer"]
pub type W = crate::W<MCYCLE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine-mode cycle counter, low half  
 Counts up once per cycle, when `mcountinhibit.cy` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`mcycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCYCLE_SPEC;
impl crate::RegisterSpec for MCYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcycle::R`](R) reader structure"]
impl crate::Readable for MCYCLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcycle::W`](W) writer structure"]
impl crate::Writable for MCYCLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCYCLE to value 0"]
impl crate::Resettable for MCYCLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
