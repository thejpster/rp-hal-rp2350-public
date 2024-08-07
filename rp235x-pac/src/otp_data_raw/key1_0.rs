#[doc = "Register `KEY1_0` reader"]
pub type R = crate::R<KEY1_0_SPEC>;
#[doc = "Register `KEY1_0` writer"]
pub type W = crate::W<KEY1_0_SPEC>;
#[doc = "Field `KEY1_0` reader - "]
pub type KEY1_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn key1_0(&self) -> KEY1_0_R {
        KEY1_0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 15:0 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_0_SPEC;
impl crate::RegisterSpec for KEY1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_0::R`](R) reader structure"]
impl crate::Readable for KEY1_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key1_0::W`](W) writer structure"]
impl crate::Writable for KEY1_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY1_0 to value 0"]
impl crate::Resettable for KEY1_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
