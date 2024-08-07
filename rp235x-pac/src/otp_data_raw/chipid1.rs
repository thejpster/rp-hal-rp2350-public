#[doc = "Register `CHIPID1` reader"]
pub type R = crate::R<CHIPID1_SPEC>;
#[doc = "Register `CHIPID1` writer"]
pub type W = crate::W<CHIPID1_SPEC>;
#[doc = "Field `CHIPID1` reader - "]
pub type CHIPID1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn chipid1(&self) -> CHIPID1_R {
        CHIPID1_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 31:16 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID1_SPEC;
impl crate::RegisterSpec for CHIPID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid1::R`](R) reader structure"]
impl crate::Readable for CHIPID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chipid1::W`](W) writer structure"]
impl crate::Writable for CHIPID1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID1 to value 0"]
impl crate::Resettable for CHIPID1_SPEC {
    const RESET_VALUE: u32 = 0;
}
