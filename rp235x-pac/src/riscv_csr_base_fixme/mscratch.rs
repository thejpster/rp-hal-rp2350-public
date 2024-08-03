#[doc = "Register `MSCRATCH` reader"]
pub type R = crate::R<MSCRATCH_SPEC>;
#[doc = "Register `MSCRATCH` writer"]
pub type W = crate::W<MSCRATCH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scratch register for machine trap handlers.  

 32-bit read/write register with no specific hardware function. Software may use this to do a fast save/restore of a core register in a trap handler.  

You can [`read`](crate::Reg::read) this register and get [`mscratch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mscratch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSCRATCH_SPEC;
impl crate::RegisterSpec for MSCRATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mscratch::R`](R) reader structure"]
impl crate::Readable for MSCRATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mscratch::W`](W) writer structure"]
impl crate::Writable for MSCRATCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCRATCH to value 0"]
impl crate::Resettable for MSCRATCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
