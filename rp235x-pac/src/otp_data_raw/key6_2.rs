#[doc = "Register `KEY6_2` reader"]
pub type R = crate::R<KEY6_2_SPEC>;
#[doc = "Register `KEY6_2` writer"]
pub type W = crate::W<KEY6_2_SPEC>;
#[doc = "Field `KEY6_2` reader - "]
pub type KEY6_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key6_2(&self) -> KEY6_2_R {
        KEY6_2_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 47:32 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY6_2_SPEC;
impl crate::RegisterSpec for KEY6_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key6_2::R`](R) reader structure"]
impl crate::Readable for KEY6_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key6_2::W`](W) writer structure"]
impl crate::Writable for KEY6_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY6_2 to value 0"]
impl crate::Resettable for KEY6_2_SPEC {
    const RESET_VALUE: u16 = 0;
}
