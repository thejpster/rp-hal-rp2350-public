#[doc = "Register `RND_SOURCE_ENABLE` reader"]
pub type R = crate::R<RND_SOURCE_ENABLE_SPEC>;
#[doc = "Register `RND_SOURCE_ENABLE` writer"]
pub type W = crate::W<RND_SOURCE_ENABLE_SPEC>;
#[doc = "Field `RND_SRC_EN` reader - * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
pub type RND_SRC_EN_R = crate::BitReader;
#[doc = "Field `RND_SRC_EN` writer - * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
pub type RND_SRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
    #[inline(always)]
    pub fn rnd_src_en(&self) -> RND_SRC_EN_R {
        RND_SRC_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - * 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_src_en(&mut self) -> RND_SRC_EN_W<RND_SOURCE_ENABLE_SPEC> {
        RND_SRC_EN_W::new(self, 0)
    }
}
#[doc = "Enable signal for the random source.  

You can [`read`](crate::Reg::read) this register and get [`rnd_source_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_source_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RND_SOURCE_ENABLE_SPEC;
impl crate::RegisterSpec for RND_SOURCE_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_source_enable::R`](R) reader structure"]
impl crate::Readable for RND_SOURCE_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rnd_source_enable::W`](W) writer structure"]
impl crate::Writable for RND_SOURCE_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RND_SOURCE_ENABLE to value 0"]
impl crate::Resettable for RND_SOURCE_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
