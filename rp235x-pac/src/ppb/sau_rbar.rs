#[doc = "Register `SAU_RBAR` reader"]
pub type R = crate::R<SAU_RBAR_SPEC>;
#[doc = "Register `SAU_RBAR` writer"]
pub type W = crate::W<SAU_RBAR_SPEC>;
#[doc = "Field `BADDR` reader - Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BADDR_R = crate::FieldReader<u32>;
#[doc = "Field `BADDR` writer - Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    pub fn baddr(&self) -> BADDR_R {
        BADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn baddr(&mut self) -> BADDR_W<SAU_RBAR_SPEC> {
        BADDR_W::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region  

You can [`read`](crate::Reg::read) this register and get [`sau_rbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_rbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAU_RBAR_SPEC;
impl crate::RegisterSpec for SAU_RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sau_rbar::R`](R) reader structure"]
impl crate::Readable for SAU_RBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sau_rbar::W`](W) writer structure"]
impl crate::Writable for SAU_RBAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAU_RBAR to value 0"]
impl crate::Resettable for SAU_RBAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
