#[doc = "Register `SAMPLE_CNT1` reader"]
pub type R = crate::R<SAMPLE_CNT1_SPEC>;
#[doc = "Register `SAMPLE_CNT1` writer"]
pub type W = crate::W<SAMPLE_CNT1_SPEC>;
#[doc = "Field `SAMPLE_CNTR1` reader - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note! If the Von-Neuman is bypassed, the minimum value for sample counter must not be less then decimal seventeen"]
pub type SAMPLE_CNTR1_R = crate::FieldReader<u32>;
#[doc = "Field `SAMPLE_CNTR1` writer - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note! If the Von-Neuman is bypassed, the minimum value for sample counter must not be less then decimal seventeen"]
pub type SAMPLE_CNTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note! If the Von-Neuman is bypassed, the minimum value for sample counter must not be less then decimal seventeen"]
    #[inline(always)]
    pub fn sample_cntr1(&self) -> SAMPLE_CNTR1_R {
        SAMPLE_CNTR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the number of rng_clk cycles between two consecutive ring oscillator samples. Note! If the Von-Neuman is bypassed, the minimum value for sample counter must not be less then decimal seventeen"]
    #[inline(always)]
    #[must_use]
    pub fn sample_cntr1(&mut self) -> SAMPLE_CNTR1_W<SAMPLE_CNT1_SPEC> {
        SAMPLE_CNTR1_W::new(self, 0)
    }
}
#[doc = "Counts clocks between sampling of random bit.  

You can [`read`](crate::Reg::read) this register and get [`sample_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_CNT1_SPEC;
impl crate::RegisterSpec for SAMPLE_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_cnt1::R`](R) reader structure"]
impl crate::Readable for SAMPLE_CNT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_cnt1::W`](W) writer structure"]
impl crate::Writable for SAMPLE_CNT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLE_CNT1 to value 0xffff"]
impl crate::Resettable for SAMPLE_CNT1_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
