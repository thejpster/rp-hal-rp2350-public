#[doc = "Register `KEY1_2` reader"]
pub type R = crate::R<KEY1_2_SPEC>;
#[doc = "Register `KEY1_2` writer"]
pub type W = crate::W<KEY1_2_SPEC>;
#[doc = "Field `KEY1_2` reader - "]
pub type KEY1_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key1_2(&self) -> KEY1_2_R {
        KEY1_2_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 47:32 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_2_SPEC;
impl crate::RegisterSpec for KEY1_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key1_2::R`](R) reader structure"]
impl crate::Readable for KEY1_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key1_2::W`](W) writer structure"]
impl crate::Writable for KEY1_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY1_2 to value 0"]
impl crate::Resettable for KEY1_2_SPEC {
    const RESET_VALUE: u16 = 0;
}
