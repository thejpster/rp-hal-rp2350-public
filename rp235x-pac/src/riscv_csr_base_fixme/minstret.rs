#[doc = "Register `MINSTRET` reader"]
pub type R = crate::R<MINSTRET_SPEC>;
#[doc = "Register `MINSTRET` writer"]
pub type W = crate::W<MINSTRET_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Machine-mode instruction retire counter, low half  
 Counts up once per instruction, when `mcountinhibit.ir` is 0. Disabled by default to save power.  

You can [`read`](crate::Reg::read) this register and get [`minstret::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`minstret::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MINSTRET_SPEC;
impl crate::RegisterSpec for MINSTRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`minstret::R`](R) reader structure"]
impl crate::Readable for MINSTRET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`minstret::W`](W) writer structure"]
impl crate::Writable for MINSTRET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MINSTRET to value 0"]
impl crate::Resettable for MINSTRET_SPEC {
    const RESET_VALUE: u32 = 0;
}
