#[doc = "Register `PROGBUF1` reader"]
pub type R = crate::R<PROGBUF1_SPEC>;
#[doc = "Register `PROGBUF1` writer"]
pub type W = crate::W<PROGBUF1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)  

You can [`read`](crate::Reg::read) this register and get [`progbuf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROGBUF1_SPEC;
impl crate::RegisterSpec for PROGBUF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`progbuf1::R`](R) reader structure"]
impl crate::Readable for PROGBUF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`progbuf1::W`](W) writer structure"]
impl crate::Writable for PROGBUF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROGBUF1 to value 0"]
impl crate::Resettable for PROGBUF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
