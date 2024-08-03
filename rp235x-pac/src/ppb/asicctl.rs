#[doc = "Register `ASICCTL` reader"]
pub type R = crate::R<ASICCTL_SPEC>;
#[doc = "Register `ASICCTL` writer"]
pub type W = crate::W<ASICCTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "External Multiplexer Control register  

You can [`read`](crate::Reg::read) this register and get [`asicctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asicctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASICCTL_SPEC;
impl crate::RegisterSpec for ASICCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asicctl::R`](R) reader structure"]
impl crate::Readable for ASICCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asicctl::W`](W) writer structure"]
impl crate::Writable for ASICCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASICCTL to value 0"]
impl crate::Resettable for ASICCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
