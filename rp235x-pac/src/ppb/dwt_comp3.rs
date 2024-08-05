#[doc = "Register `DWT_COMP3` reader"]
pub type R = crate::R<DWT_COMP3_SPEC>;
#[doc = "Register `DWT_COMP3` writer"]
pub type W = crate::W<DWT_COMP3_SPEC>;
#[doc = "Field `DWT_COMP3` reader - "]
pub type DWT_COMP3_R = crate::FieldReader<u32>;
#[doc = "Field `DWT_COMP3` writer - "]
pub type DWT_COMP3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dwt_comp3(&self) -> DWT_COMP3_R {
        DWT_COMP3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dwt_comp3(&mut self) -> DWT_COMP3_W<DWT_COMP3_SPEC> {
        DWT_COMP3_W::new(self, 0)
    }
}
#[doc = "Provides a reference value for use by watchpoint comparator 3  

You can [`read`](crate::Reg::read) this register and get [`dwt_comp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwt_comp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWT_COMP3_SPEC;
impl crate::RegisterSpec for DWT_COMP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dwt_comp3::R`](R) reader structure"]
impl crate::Readable for DWT_COMP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dwt_comp3::W`](W) writer structure"]
impl crate::Writable for DWT_COMP3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DWT_COMP3 to value 0"]
impl crate::Resettable for DWT_COMP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
