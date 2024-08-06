#[doc = "Register `KEY5_0` reader"]
pub type R = crate::R<KEY5_0_SPEC>;
#[doc = "Register `KEY5_0` writer"]
pub type W = crate::W<KEY5_0_SPEC>;
#[doc = "Field `KEY5_0` reader - "]
pub type KEY5_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key5_0(&self) -> KEY5_0_R {
        KEY5_0_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 15:0 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY5_0_SPEC;
impl crate::RegisterSpec for KEY5_0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`key5_0::R`](R) reader structure"]
impl crate::Readable for KEY5_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key5_0::W`](W) writer structure"]
impl crate::Writable for KEY5_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KEY5_0 to value 0"]
impl crate::Resettable for KEY5_0_SPEC {
    const RESET_VALUE: u16 = 0;
}
