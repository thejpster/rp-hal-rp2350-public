#[doc = "Register `BIST` reader"]
pub type R = crate::R<BIST_SPEC>;
#[doc = "Register `BIST` writer"]
pub type W = crate::W<BIST_SPEC>;
#[doc = "Field `CNT` reader - Number of locations that have at least one leaky bit. Note: This count is true only if the BIST was initiated without the fix option."]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_MAX` reader - The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
pub type CNT_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_MAX` writer - The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
pub type CNT_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CNT_ENA` reader - Enable the counter before the BIST function is initiated"]
pub type CNT_ENA_R = crate::BitReader;
#[doc = "Field `CNT_ENA` writer - Enable the counter before the BIST function is initiated"]
pub type CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_CLR` writer - Clear counter before use"]
pub type CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_FAIL` reader - Flag if the count of address locations with at least one leaky bit exceeds cnt_max"]
pub type CNT_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:12 - Number of locations that have at least one leaky bit. Note: This count is true only if the BIST was initiated without the fix option."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
    #[inline(always)]
    pub fn cnt_max(&self) -> CNT_MAX_R {
        CNT_MAX_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Enable the counter before the BIST function is initiated"]
    #[inline(always)]
    pub fn cnt_ena(&self) -> CNT_ENA_R {
        CNT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Flag if the count of address locations with at least one leaky bit exceeds cnt_max"]
    #[inline(always)]
    pub fn cnt_fail(&self) -> CNT_FAIL_R {
        CNT_FAIL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:27 - The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_max(&mut self) -> CNT_MAX_W<BIST_SPEC> {
        CNT_MAX_W::new(self, 16)
    }
    #[doc = "Bit 28 - Enable the counter before the BIST function is initiated"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ena(&mut self) -> CNT_ENA_W<BIST_SPEC> {
        CNT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear counter before use"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_clr(&mut self) -> CNT_CLR_W<BIST_SPEC> {
        CNT_CLR_W::new(self, 29)
    }
}
#[doc = "During BIST, count address locations that have at least one leaky bit  

You can [`read`](crate::Reg::read) this register and get [`bist::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bist::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIST_SPEC;
impl crate::RegisterSpec for BIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist::R`](R) reader structure"]
impl crate::Readable for BIST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bist::W`](W) writer structure"]
impl crate::Writable for BIST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIST to value 0x0fff_0000"]
impl crate::Resettable for BIST_SPEC {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
