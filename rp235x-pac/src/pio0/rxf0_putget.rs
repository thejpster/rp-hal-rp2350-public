#[doc = "Register `RXF0_PUTGET%s` reader"]
pub type R = crate::R<RXF0_PUTGET_SPEC>;
#[doc = "Register `RXF0_PUTGET%s` writer"]
pub type W = crate::W<RXF0_PUTGET_SPEC>;
#[doc = "Field `RXF0_PUTGET0` reader - "]
pub type RXF0_PUTGET0_R = crate::FieldReader<u32>;
#[doc = "Field `RXF0_PUTGET0` writer - "]
pub type RXF0_PUTGET0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxf0_putget0(&self) -> RXF0_PUTGET0_R {
        RXF0_PUTGET0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rxf0_putget0(&mut self) -> RXF0_PUTGET0_W<RXF0_PUTGET_SPEC> {
        RXF0_PUTGET0_W::new(self, 0)
    }
}
#[doc = "Direct read/write access to entry %s of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.  

You can [`read`](crate::Reg::read) this register and get [`rxf0_putget::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0_putget::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0_PUTGET_SPEC;
impl crate::RegisterSpec for RXF0_PUTGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0_putget::R`](R) reader structure"]
impl crate::Readable for RXF0_PUTGET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf0_putget::W`](W) writer structure"]
impl crate::Writable for RXF0_PUTGET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0_PUTGET%s to value 0"]
impl crate::Resettable for RXF0_PUTGET_SPEC {
    const RESET_VALUE: u32 = 0;
}
