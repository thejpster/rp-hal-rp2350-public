#[doc = "Register `MCYCLEH` reader"]
pub type R = crate::R<MCYCLEH_SPEC>;
#[doc = "Register `MCYCLEH` writer"]
pub type W = crate::W<MCYCLEH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine-mode cycle counter, high half  
 Counts up once per 1 &lt;&lt; 32 cycles, when `mcountinhibit.cy` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`mcycleh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcycleh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCYCLEH_SPEC;
impl crate::RegisterSpec for MCYCLEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcycleh::R`](R) reader structure"]
impl crate::Readable for MCYCLEH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcycleh::W`](W) writer structure"]
impl crate::Writable for MCYCLEH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCYCLEH to value 0"]
impl crate::Resettable for MCYCLEH_SPEC {
    const RESET_VALUE: u32 = 0;
}
