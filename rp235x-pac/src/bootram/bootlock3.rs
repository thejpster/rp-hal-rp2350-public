#[doc = "Register `BOOTLOCK3` reader"]
pub type R = crate::R<BOOTLOCK3_SPEC>;
#[doc = "Register `BOOTLOCK3` writer"]
pub type W = crate::W<BOOTLOCK3_SPEC>;
#[doc = "Field `BOOTLOCK3` reader - "]
pub type BOOTLOCK3_R = crate::FieldReader<u32>;
#[doc = "Field `BOOTLOCK3` writer - "]
pub type BOOTLOCK3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootlock3(&self) -> BOOTLOCK3_R {
        BOOTLOCK3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bootlock3(&mut self) -> BOOTLOCK3_W<BOOTLOCK3_SPEC> {
        BOOTLOCK3_W::new(self, 0)
    }
}
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOCK3_SPEC;
impl crate::RegisterSpec for BOOTLOCK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootlock3::R`](R) reader structure"]
impl crate::Readable for BOOTLOCK3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootlock3::W`](W) writer structure"]
impl crate::Writable for BOOTLOCK3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOCK3 to value 0"]
impl crate::Resettable for BOOTLOCK3_SPEC {
    const RESET_VALUE: u32 = 0;
}
