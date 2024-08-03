#[doc = "Register `DOORBELL_OUT_SET` reader"]
pub type R = crate::R<DOORBELL_OUT_SET_SPEC>;
#[doc = "Register `DOORBELL_OUT_SET` writer"]
pub type W = crate::W<DOORBELL_OUT_SET_SPEC>;
#[doc = "Field `DOORBELL_OUT_SET` reader - "]
pub type DOORBELL_OUT_SET_R = crate::FieldReader;
#[doc = "Field `DOORBELL_OUT_SET` writer - "]
pub type DOORBELL_OUT_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn doorbell_out_set(&self) -> DOORBELL_OUT_SET_R {
        DOORBELL_OUT_SET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell_out_set(&mut self) -> DOORBELL_OUT_SET_W<DOORBELL_OUT_SET_SPEC> {
        DOORBELL_OUT_SET_W::new(self, 0)
    }
}
#[doc = "Trigger a doorbell interrupt on the opposite core.  

 Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt.  

 Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status.  

You can [`read`](crate::Reg::read) this register and get [`doorbell_out_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell_out_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOORBELL_OUT_SET_SPEC;
impl crate::RegisterSpec for DOORBELL_OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doorbell_out_set::R`](R) reader structure"]
impl crate::Readable for DOORBELL_OUT_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doorbell_out_set::W`](W) writer structure"]
impl crate::Writable for DOORBELL_OUT_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOORBELL_OUT_SET to value 0"]
impl crate::Resettable for DOORBELL_OUT_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
