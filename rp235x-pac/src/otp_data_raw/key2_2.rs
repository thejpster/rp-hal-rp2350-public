#[doc = "Register `KEY2_2` reader"]
pub type R = crate::R<KEY2_2_SPEC>;
#[doc = "Register `KEY2_2` writer"]
pub type W = crate::W<KEY2_2_SPEC>;
#[doc = "Field `KEY2_2` reader - "]
pub type KEY2_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn key2_2(&self) -> KEY2_2_R {
        KEY2_2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 47:32 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY2_2_SPEC;
impl crate::RegisterSpec for KEY2_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2_2::R`](R) reader structure"]
impl crate::Readable for KEY2_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key2_2::W`](W) writer structure"]
impl crate::Writable for KEY2_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2_2 to value 0"]
impl crate::Resettable for KEY2_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
