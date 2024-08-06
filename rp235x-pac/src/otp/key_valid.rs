#[doc = "Register `KEY_VALID` reader"]
pub type R = crate::R<KEY_VALID_SPEC>;
#[doc = "Register `KEY_VALID` writer"]
pub type W = crate::W<KEY_VALID_SPEC>;
#[doc = "Field `KEY_VALID` reader - "]
pub type KEY_VALID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn key_valid(&self) -> KEY_VALID_R {
        KEY_VALID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Which keys were valid (enrolled) at boot time  

You can [`read`](crate::Reg::read) this register and get [`key_valid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_valid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_VALID_SPEC;
impl crate::RegisterSpec for KEY_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_valid::R`](R) reader structure"]
impl crate::Readable for KEY_VALID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_valid::W`](W) writer structure"]
impl crate::Writable for KEY_VALID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_VALID to value 0"]
impl crate::Resettable for KEY_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
