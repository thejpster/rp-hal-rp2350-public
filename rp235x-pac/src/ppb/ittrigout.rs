#[doc = "Register `ITTRIGOUT` reader"]
pub type R = crate::R<ITTRIGOUT_SPEC>;
#[doc = "Register `ITTRIGOUT` writer"]
pub type W = crate::W<ITTRIGOUT_SPEC>;
#[doc = "Field `CTTRIGOUT` reader - Sets the value of the ctitrigout outputs"]
pub type CTTRIGOUT_R = crate::FieldReader;
#[doc = "Field `CTTRIGOUT` writer - Sets the value of the ctitrigout outputs"]
pub type CTTRIGOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sets the value of the ctitrigout outputs"]
    #[inline(always)]
    pub fn cttrigout(&self) -> CTTRIGOUT_R {
        CTTRIGOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sets the value of the ctitrigout outputs"]
    #[inline(always)]
    #[must_use]
    pub fn cttrigout(&mut self) -> CTTRIGOUT_W<ITTRIGOUT_SPEC> {
        CTTRIGOUT_W::new(self, 0)
    }
}
#[doc = "Integration Test Trigger Output register  

You can [`read`](crate::Reg::read) this register and get [`ittrigout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITTRIGOUT_SPEC;
impl crate::RegisterSpec for ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrigout::R`](R) reader structure"]
impl crate::Readable for ITTRIGOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ittrigout::W`](W) writer structure"]
impl crate::Writable for ITTRIGOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for ITTRIGOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
