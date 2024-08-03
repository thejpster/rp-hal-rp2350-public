#[doc = "Register `COUNT` reader"]
pub type R = crate::R<COUNT_SPEC>;
#[doc = "Field `PROC0_COUNT` reader - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
pub type PROC0_COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn proc0_count(&self) -> PROC0_COUNT_R {
        PROC0_COUNT_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT_SPEC;
impl crate::RegisterSpec for COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for COUNT_SPEC {}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for COUNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
