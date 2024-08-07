#[doc = "Register `KEY5_2` reader"]
pub type R = crate::R<KEY5_2_SPEC>;
#[doc = "Register `KEY5_2` writer"]
pub type W = crate::W<KEY5_2_SPEC>;
#[doc = "Field `KEY5_2` reader - "]
pub type KEY5_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn key5_2(&self) -> KEY5_2_R {
        KEY5_2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 47:32 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY5_2_SPEC;
impl crate::RegisterSpec for KEY5_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_2::R`](R) reader structure"]
impl crate::Readable for KEY5_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key5_2::W`](W) writer structure"]
impl crate::Writable for KEY5_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY5_2 to value 0"]
impl crate::Resettable for KEY5_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
