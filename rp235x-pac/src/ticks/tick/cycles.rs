#[doc = "Register `CYCLES` reader"]
pub type R = crate::R<CYCLES_SPEC>;
#[doc = "Register `CYCLES` writer"]
pub type W = crate::W<CYCLES_SPEC>;
#[doc = "Field `PROC0_CYCLES` reader - Total number of clk_tick cycles before the next tick."]
pub type PROC0_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `PROC0_CYCLES` writer - Total number of clk_tick cycles before the next tick."]
pub type PROC0_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn proc0_cycles(&self) -> PROC0_CYCLES_R {
        PROC0_CYCLES_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    #[must_use]
    pub fn proc0_cycles(&mut self) -> PROC0_CYCLES_W<CYCLES_SPEC> {
        PROC0_CYCLES_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`cycles::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycles::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CYCLES_SPEC;
impl crate::RegisterSpec for CYCLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycles::R`](R) reader structure"]
impl crate::Readable for CYCLES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cycles::W`](W) writer structure"]
impl crate::Writable for CYCLES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CYCLES to value 0"]
impl crate::Resettable for CYCLES_SPEC {
    const RESET_VALUE: u32 = 0;
}
