#[doc = "Register `CHIPID3` reader"]
pub type R = crate::R<CHIPID3_SPEC>;
#[doc = "Register `CHIPID3` writer"]
pub type W = crate::W<CHIPID3_SPEC>;
#[doc = "Field `CHIPID3` reader - "]
pub type CHIPID3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn chipid3(&self) -> CHIPID3_R {
        CHIPID3_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 63:48 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID3_SPEC;
impl crate::RegisterSpec for CHIPID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid3::R`](R) reader structure"]
impl crate::Readable for CHIPID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chipid3::W`](W) writer structure"]
impl crate::Writable for CHIPID3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID3 to value 0"]
impl crate::Resettable for CHIPID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
