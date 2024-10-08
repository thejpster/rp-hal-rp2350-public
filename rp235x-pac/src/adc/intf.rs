#[doc = "Register `INTF` reader"]
pub type R = crate::R<INTF_SPEC>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<INTF_SPEC>;
#[doc = "Field `FIFO` reader - Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
pub type FIFO_R = crate::BitReader;
#[doc = "Field `FIFO` writer - Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
pub type FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FIFO_W<INTF_SPEC> {
        FIFO_W::new(self, 0)
    }
}
#[doc = "Interrupt Force  

You can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
