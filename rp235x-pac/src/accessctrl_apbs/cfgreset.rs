#[doc = "Register `CFGRESET` reader"]
pub type R = crate::R<CFGRESET_SPEC>;
#[doc = "Register `CFGRESET` writer"]
pub type W = crate::W<CFGRESET_SPEC>;
#[doc = "Field `CFGRESET` reader - "]
pub type CFGRESET_R = crate::BitReader;
#[doc = "Field `CFGRESET` writer - "]
pub type CFGRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfgreset(&self) -> CFGRESET_R {
        CFGRESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cfgreset(&mut self) -> CFGRESET_W<CFGRESET_SPEC> {
        CFGRESET_W::new(self, 0)
    }
}
#[doc = "Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers.  

 This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path.  

 Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents.  

You can [`read`](crate::Reg::read) this register and get [`cfgreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRESET_SPEC;
impl crate::RegisterSpec for CFGRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgreset::R`](R) reader structure"]
impl crate::Readable for CFGRESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgreset::W`](W) writer structure"]
impl crate::Writable for CFGRESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGRESET to value 0"]
impl crate::Resettable for CFGRESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
