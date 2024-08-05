#[doc = "Register `KEY1_5` reader"]
pub type R = crate::R<KEY1_5_SPEC>;
#[doc = "Register `KEY1_5` writer"]
pub type W = crate::W<KEY1_5_SPEC>;
#[doc = "Field `KEY1_5` reader - "]
pub type KEY1_5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key1_5(&self) -> KEY1_5_R {
        KEY1_5_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 95:80 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_5_SPEC;
impl crate::RegisterSpec for KEY1_5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key1_5::R`](R) reader structure"]
impl crate::Readable for KEY1_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key1_5::W`](W) writer structure"]
impl crate::Writable for KEY1_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY1_5 to value 0"]
impl crate::Resettable for KEY1_5_SPEC {
    const RESET_VALUE: u16 = 0;
}
