#[doc = "Register `CHIPID2` reader"]
pub type R = crate::R<CHIPID2_SPEC>;
#[doc = "Register `CHIPID2` writer"]
pub type W = crate::W<CHIPID2_SPEC>;
#[doc = "Field `CHIPID2` reader - "]
pub type CHIPID2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn chipid2(&self) -> CHIPID2_R {
        CHIPID2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 47:32 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID2_SPEC;
impl crate::RegisterSpec for CHIPID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid2::R`](R) reader structure"]
impl crate::Readable for CHIPID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chipid2::W`](W) writer structure"]
impl crate::Writable for CHIPID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID2 to value 0"]
impl crate::Resettable for CHIPID2_SPEC {
    const RESET_VALUE: u32 = 0;
}
