#[doc = "Register `PACKAGE_SEL` reader"]
pub type R = crate::R<PACKAGE_SEL_SPEC>;
#[doc = "Register `PACKAGE_SEL` writer"]
pub type W = crate::W<PACKAGE_SEL_SPEC>;
#[doc = "Field `PACKAGE_SEL` reader - "]
pub type PACKAGE_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn package_sel(&self) -> PACKAGE_SEL_R {
        PACKAGE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`package_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`package_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PACKAGE_SEL_SPEC;
impl crate::RegisterSpec for PACKAGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`package_sel::R`](R) reader structure"]
impl crate::Readable for PACKAGE_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`package_sel::W`](W) writer structure"]
impl crate::Writable for PACKAGE_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PACKAGE_SEL to value 0"]
impl crate::Resettable for PACKAGE_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
