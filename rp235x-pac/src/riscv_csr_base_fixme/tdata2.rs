#[doc = "Register `TDATA2` reader"]
pub type R = crate::R<TDATA2_SPEC>;
#[doc = "Register `TDATA2` writer"]
pub type W = crate::W<TDATA2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Trigger configuration data 2  

 Contains the address for instruction address triggers (hardware breakpoints)  

You can [`read`](crate::Reg::read) this register and get [`tdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDATA2_SPEC;
impl crate::RegisterSpec for TDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdata2::R`](R) reader structure"]
impl crate::Readable for TDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdata2::W`](W) writer structure"]
impl crate::Writable for TDATA2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDATA2 to value 0"]
impl crate::Resettable for TDATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
