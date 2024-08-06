#[doc = "Register `KEY6_4` reader"]
pub type R = crate::R<KEY6_4_SPEC>;
#[doc = "Register `KEY6_4` writer"]
pub type W = crate::W<KEY6_4_SPEC>;
#[doc = "Field `KEY6_4` reader - "]
pub type KEY6_4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key6_4(&self) -> KEY6_4_R {
        KEY6_4_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 79:64 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY6_4_SPEC;
impl crate::RegisterSpec for KEY6_4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key6_4::R`](R) reader structure"]
impl crate::Readable for KEY6_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key6_4::W`](W) writer structure"]
impl crate::Writable for KEY6_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY6_4 to value 0"]
impl crate::Resettable for KEY6_4_SPEC {
    const RESET_VALUE: u16 = 0;
}
