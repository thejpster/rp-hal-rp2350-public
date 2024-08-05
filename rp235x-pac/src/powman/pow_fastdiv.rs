#[doc = "Register `POW_FASTDIV` reader"]
pub type R = crate::R<POW_FASTDIV_SPEC>;
#[doc = "Register `POW_FASTDIV` writer"]
pub type W = crate::W<POW_FASTDIV_SPEC>;
#[doc = "Field `POW_FASTDIV` reader - divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
pub type POW_FASTDIV_R = crate::FieldReader<u16>;
#[doc = "Field `POW_FASTDIV` writer - divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
pub type POW_FASTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
    #[inline(always)]
    pub fn pow_fastdiv(&self) -> POW_FASTDIV_R {
        POW_FASTDIV_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
    #[inline(always)]
    #[must_use]
    pub fn pow_fastdiv(&mut self) -> POW_FASTDIV_W<POW_FASTDIV_SPEC> {
        POW_FASTDIV_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`pow_fastdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pow_fastdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POW_FASTDIV_SPEC;
impl crate::RegisterSpec for POW_FASTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pow_fastdiv::R`](R) reader structure"]
impl crate::Readable for POW_FASTDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pow_fastdiv::W`](W) writer structure"]
impl crate::Writable for POW_FASTDIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POW_FASTDIV to value 0x40"]
impl crate::Resettable for POW_FASTDIV_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
