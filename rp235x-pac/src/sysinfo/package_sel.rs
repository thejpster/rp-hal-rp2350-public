#[doc = "Register `PACKAGE_SEL` reader"]
pub type R = crate::R<PACKAGE_SEL_SPEC>;
#[doc = "Field `PACKAGE_SEL` reader - "]
pub type PACKAGE_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn package_sel(&self) -> PACKAGE_SEL_R {
        PACKAGE_SEL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`package_sel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PACKAGE_SEL_SPEC;
impl crate::RegisterSpec for PACKAGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`package_sel::R`](R) reader structure"]
impl crate::Readable for PACKAGE_SEL_SPEC {}
#[doc = "`reset()` method sets PACKAGE_SEL to value 0"]
impl crate::Resettable for PACKAGE_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
