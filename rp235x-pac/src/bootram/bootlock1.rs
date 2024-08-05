#[doc = "Register `BOOTLOCK1` reader"]
pub type R = crate::R<BOOTLOCK1_SPEC>;
#[doc = "Register `BOOTLOCK1` writer"]
pub type W = crate::W<BOOTLOCK1_SPEC>;
#[doc = "Field `BOOTLOCK1` reader - "]
pub type BOOTLOCK1_R = crate::FieldReader<u32>;
#[doc = "Field `BOOTLOCK1` writer - "]
pub type BOOTLOCK1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootlock1(&self) -> BOOTLOCK1_R {
        BOOTLOCK1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bootlock1(&mut self) -> BOOTLOCK1_W<BOOTLOCK1_SPEC> {
        BOOTLOCK1_W::new(self, 0)
    }
}
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOCK1_SPEC;
impl crate::RegisterSpec for BOOTLOCK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootlock1::R`](R) reader structure"]
impl crate::Readable for BOOTLOCK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootlock1::W`](W) writer structure"]
impl crate::Writable for BOOTLOCK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOCK1 to value 0"]
impl crate::Resettable for BOOTLOCK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
