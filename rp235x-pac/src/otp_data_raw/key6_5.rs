#[doc = "Register `KEY6_5` reader"]
pub type R = crate::R<KEY6_5_SPEC>;
#[doc = "Register `KEY6_5` writer"]
pub type W = crate::W<KEY6_5_SPEC>;
#[doc = "Field `KEY6_5` reader - "]
pub type KEY6_5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn key6_5(&self) -> KEY6_5_R {
        KEY6_5_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 95:80 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY6_5_SPEC;
impl crate::RegisterSpec for KEY6_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6_5::R`](R) reader structure"]
impl crate::Readable for KEY6_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key6_5::W`](W) writer structure"]
impl crate::Writable for KEY6_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY6_5 to value 0"]
impl crate::Resettable for KEY6_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
