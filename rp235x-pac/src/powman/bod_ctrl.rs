#[doc = "Register `BOD_CTRL` reader"]
pub type R = crate::R<BOD_CTRL_SPEC>;
#[doc = "Register `BOD_CTRL` writer"]
pub type W = crate::W<BOD_CTRL_SPEC>;
#[doc = "Field `ISOLATE` reader - isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
pub type ISOLATE_R = crate::BitReader;
#[doc = "Field `ISOLATE` writer - isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
pub type ISOLATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub fn isolate(&self) -> ISOLATE_R {
        ISOLATE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    #[must_use]
    pub fn isolate(&mut self) -> ISOLATE_W<BOD_CTRL_SPEC> {
        ISOLATE_W::new(self, 12)
    }
}
#[doc = "Brown-out Detection Control  

You can [`read`](crate::Reg::read) this register and get [`bod_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOD_CTRL_SPEC;
impl crate::RegisterSpec for BOD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod_ctrl::R`](R) reader structure"]
impl crate::Readable for BOD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bod_ctrl::W`](W) writer structure"]
impl crate::Writable for BOD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOD_CTRL to value 0"]
impl crate::Resettable for BOD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
