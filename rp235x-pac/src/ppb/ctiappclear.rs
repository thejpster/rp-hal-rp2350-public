#[doc = "Register `CTIAPPCLEAR` reader"]
pub type R = crate::R<CTIAPPCLEAR_SPEC>;
#[doc = "Register `CTIAPPCLEAR` writer"]
pub type W = crate::W<CTIAPPCLEAR_SPEC>;
#[doc = "Field `APPCLEAR` reader - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type APPCLEAR_R = crate::FieldReader;
#[doc = "Field `APPCLEAR` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type APPCLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear(&self) -> APPCLEAR_R {
        APPCLEAR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    #[must_use]
    pub fn appclear(&mut self) -> APPCLEAR_W<CTIAPPCLEAR_SPEC> {
        APPCLEAR_W::new(self, 0)
    }
}
#[doc = "CTI Application Trigger Clear Register  

You can [`read`](crate::Reg::read) this register and get [`ctiappclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIAPPCLEAR_SPEC;
impl crate::RegisterSpec for CTIAPPCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiappclear::R`](R) reader structure"]
impl crate::Readable for CTIAPPCLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiappclear::W`](W) writer structure"]
impl crate::Writable for CTIAPPCLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIAPPCLEAR to value 0"]
impl crate::Resettable for CTIAPPCLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
