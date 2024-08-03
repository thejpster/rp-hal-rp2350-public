#[doc = "Register `CTIGATE` reader"]
pub type R = crate::R<CTIGATE_SPEC>;
#[doc = "Register `CTIGATE` writer"]
pub type W = crate::W<CTIGATE_SPEC>;
#[doc = "Field `CTIGATEEN0` reader - Enable ctichout0. Set to 0 to disable channel propagation."]
pub type CTIGATEEN0_R = crate::BitReader;
#[doc = "Field `CTIGATEEN0` writer - Enable ctichout0. Set to 0 to disable channel propagation."]
pub type CTIGATEEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN1` reader - Enable ctichout1. Set to 0 to disable channel propagation."]
pub type CTIGATEEN1_R = crate::BitReader;
#[doc = "Field `CTIGATEEN1` writer - Enable ctichout1. Set to 0 to disable channel propagation."]
pub type CTIGATEEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN2` reader - Enable ctichout2. Set to 0 to disable channel propagation."]
pub type CTIGATEEN2_R = crate::BitReader;
#[doc = "Field `CTIGATEEN2` writer - Enable ctichout2. Set to 0 to disable channel propagation."]
pub type CTIGATEEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN3` reader - Enable ctichout3. Set to 0 to disable channel propagation."]
pub type CTIGATEEN3_R = crate::BitReader;
#[doc = "Field `CTIGATEEN3` writer - Enable ctichout3. Set to 0 to disable channel propagation."]
pub type CTIGATEEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ctichout0. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen0(&self) -> CTIGATEEN0_R {
        CTIGATEEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ctichout1. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen1(&self) -> CTIGATEEN1_R {
        CTIGATEEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable ctichout2. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen2(&self) -> CTIGATEEN2_R {
        CTIGATEEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable ctichout3. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen3(&self) -> CTIGATEEN3_R {
        CTIGATEEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctichout0. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen0(&mut self) -> CTIGATEEN0_W<CTIGATE_SPEC> {
        CTIGATEEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ctichout1. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen1(&mut self) -> CTIGATEEN1_W<CTIGATE_SPEC> {
        CTIGATEEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable ctichout2. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen2(&mut self) -> CTIGATEEN2_W<CTIGATE_SPEC> {
        CTIGATEEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable ctichout3. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen3(&mut self) -> CTIGATEEN3_W<CTIGATE_SPEC> {
        CTIGATEEN3_W::new(self, 3)
    }
}
#[doc = "Enable CTI Channel Gate register  

You can [`read`](crate::Reg::read) this register and get [`ctigate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctigate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIGATE_SPEC;
impl crate::RegisterSpec for CTIGATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctigate::R`](R) reader structure"]
impl crate::Readable for CTIGATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctigate::W`](W) writer structure"]
impl crate::Writable for CTIGATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIGATE to value 0x0f"]
impl crate::Resettable for CTIGATE_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
