#[doc = "Register `DBGCONFIG` reader"]
pub type R = crate::R<DBGCONFIG_SPEC>;
#[doc = "Register `DBGCONFIG` writer"]
pub type W = crate::W<DBGCONFIG_SPEC>;
#[doc = "Field `DP_INSTID` reader - Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
pub type DP_INSTID_R = crate::FieldReader;
#[doc = "Field `DP_INSTID` writer - Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
pub type DP_INSTID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
    #[inline(always)]
    pub fn dp_instid(&self) -> DP_INSTID_R {
        DP_INSTID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
    #[inline(always)]
    #[must_use]
    pub fn dp_instid(&mut self) -> DP_INSTID_W<DBGCONFIG_SPEC> {
        DP_INSTID_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`dbgconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGCONFIG_SPEC;
impl crate::RegisterSpec for DBGCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgconfig::R`](R) reader structure"]
impl crate::Readable for DBGCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgconfig::W`](W) writer structure"]
impl crate::Writable for DBGCONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCONFIG to value 0"]
impl crate::Resettable for DBGCONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
