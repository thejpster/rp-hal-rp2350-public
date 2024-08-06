#[doc = "Register `TRCIDR9` reader"]
pub type R = crate::R<TRCIDR9_SPEC>;
#[doc = "Register `TRCIDR9` writer"]
pub type W = crate::W<TRCIDR9_SPEC>;
#[doc = "Field `NUMP0KEY` reader - reads as `ImpDef"]
pub type NUMP0KEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nump0key(&self) -> NUMP0KEY_R {
        NUMP0KEY_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR9  

You can [`read`](crate::Reg::read) this register and get [`trcidr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR9_SPEC;
impl crate::RegisterSpec for TRCIDR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr9::R`](R) reader structure"]
impl crate::Readable for TRCIDR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr9::W`](W) writer structure"]
impl crate::Writable for TRCIDR9_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR9 to value 0"]
impl crate::Resettable for TRCIDR9_SPEC {
    const RESET_VALUE: u32 = 0;
}
