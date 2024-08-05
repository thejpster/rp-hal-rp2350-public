#[doc = "Register `KEY2_7` reader"]
pub type R = crate::R<KEY2_7_SPEC>;
#[doc = "Register `KEY2_7` writer"]
pub type W = crate::W<KEY2_7_SPEC>;
#[doc = "Field `KEY2_7` reader - "]
pub type KEY2_7_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key2_7(&self) -> KEY2_7_R {
        KEY2_7_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 127:112 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY2_7_SPEC;
impl crate::RegisterSpec for KEY2_7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key2_7::R`](R) reader structure"]
impl crate::Readable for KEY2_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key2_7::W`](W) writer structure"]
impl crate::Writable for KEY2_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY2_7 to value 0"]
impl crate::Resettable for KEY2_7_SPEC {
    const RESET_VALUE: u16 = 0;
}
