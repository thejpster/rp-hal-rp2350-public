#[doc = "Register `DBG_PWRCFG` reader"]
pub type R = crate::R<DBG_PWRCFG_SPEC>;
#[doc = "Register `DBG_PWRCFG` writer"]
pub type W = crate::W<DBG_PWRCFG_SPEC>;
#[doc = "Field `IGNORE` reader - Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
pub type IGNORE_R = crate::BitReader;
#[doc = "Field `IGNORE` writer - Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
pub type IGNORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
    #[inline(always)]
    pub fn ignore(&self) -> IGNORE_R {
        IGNORE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
    #[inline(always)]
    #[must_use]
    pub fn ignore(&mut self) -> IGNORE_W<DBG_PWRCFG_SPEC> {
        IGNORE_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`dbg_pwrcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pwrcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PWRCFG_SPEC;
impl crate::RegisterSpec for DBG_PWRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_pwrcfg::R`](R) reader structure"]
impl crate::Readable for DBG_PWRCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_pwrcfg::W`](W) writer structure"]
impl crate::Writable for DBG_PWRCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_PWRCFG to value 0"]
impl crate::Resettable for DBG_PWRCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
