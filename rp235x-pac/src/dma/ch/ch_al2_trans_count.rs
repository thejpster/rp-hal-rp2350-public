#[doc = "Register `CH_AL2_TRANS_COUNT` reader"]
pub type R = crate::R<CH_AL2_TRANS_COUNT_SPEC>;
#[doc = "Register `CH_AL2_TRANS_COUNT` writer"]
pub type W = crate::W<CH_AL2_TRANS_COUNT_SPEC>;
#[doc = "Field `CH0_AL2_TRANS_COUNT` reader - "]
pub type CH0_AL2_TRANS_COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `CH0_AL2_TRANS_COUNT` writer - "]
pub type CH0_AL2_TRANS_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_al2_trans_count(&self) -> CH0_AL2_TRANS_COUNT_R {
        CH0_AL2_TRANS_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_al2_trans_count(&mut self) -> CH0_AL2_TRANS_COUNT_W<CH_AL2_TRANS_COUNT_SPEC> {
        CH0_AL2_TRANS_COUNT_W::new(self, 0)
    }
}
#[doc = "Alias for channel 0 TRANS_COUNT register  

You can [`read`](crate::Reg::read) this register and get [`ch_al2_trans_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_al2_trans_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_AL2_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_AL2_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_al2_trans_count::R`](R) reader structure"]
impl crate::Readable for CH_AL2_TRANS_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_al2_trans_count::W`](W) writer structure"]
impl crate::Writable for CH_AL2_TRANS_COUNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_AL2_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_AL2_TRANS_COUNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
