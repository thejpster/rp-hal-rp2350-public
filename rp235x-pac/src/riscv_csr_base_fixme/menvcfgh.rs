#[doc = "Register `MENVCFGH` reader"]
pub type R = crate::R<MENVCFGH_SPEC>;
#[doc = "Register `MENVCFGH` writer"]
pub type W = crate::W<MENVCFGH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine environment configuration register, high half  

 This register is fully reserved, as Hazard3 does not implement the relevant extensions. It is implemented as hardwired-0.  

You can [`read`](crate::Reg::read) this register and get [`menvcfgh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`menvcfgh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MENVCFGH_SPEC;
impl crate::RegisterSpec for MENVCFGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`menvcfgh::R`](R) reader structure"]
impl crate::Readable for MENVCFGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`menvcfgh::W`](W) writer structure"]
impl crate::Writable for MENVCFGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MENVCFGH to value 0"]
impl crate::Resettable for MENVCFGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
