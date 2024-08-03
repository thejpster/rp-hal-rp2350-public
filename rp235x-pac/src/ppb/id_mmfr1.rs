#[doc = "Register `ID_MMFR1` reader"]
pub type R = crate::R<ID_MMFR1_SPEC>;
#[doc = "Register `ID_MMFR1` writer"]
pub type W = crate::W<ID_MMFR1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_MMFR1_SPEC;
impl crate::RegisterSpec for ID_MMFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr1::R`](R) reader structure"]
impl crate::Readable for ID_MMFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_mmfr1::W`](W) writer structure"]
impl crate::Writable for ID_MMFR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_MMFR1 to value 0"]
impl crate::Resettable for ID_MMFR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
