#[doc = "Register `BOOTLOCK0` reader"]
pub type R = crate::R<BOOTLOCK0_SPEC>;
#[doc = "Register `BOOTLOCK0` writer"]
pub type W = crate::W<BOOTLOCK0_SPEC>;
#[doc = "Field `BOOTLOCK0` reader - "]
pub type BOOTLOCK0_R = crate::FieldReader<u32>;
#[doc = "Field `BOOTLOCK0` writer - "]
pub type BOOTLOCK0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootlock0(&self) -> BOOTLOCK0_R {
        BOOTLOCK0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bootlock0(&mut self) -> BOOTLOCK0_W<BOOTLOCK0_SPEC> {
        BOOTLOCK0_W::new(self, 0)
    }
}
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOCK0_SPEC;
impl crate::RegisterSpec for BOOTLOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootlock0::R`](R) reader structure"]
impl crate::Readable for BOOTLOCK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootlock0::W`](W) writer structure"]
impl crate::Writable for BOOTLOCK0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOCK0 to value 0"]
impl crate::Resettable for BOOTLOCK0_SPEC {
    const RESET_VALUE: u32 = 0;
}
