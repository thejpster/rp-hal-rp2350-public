#[doc = "Register `DCRDR` reader"]
pub type R = crate::R<DCRDR_SPEC>;
#[doc = "Register `DCRDR` writer"]
pub type W = crate::W<DCRDR_SPEC>;
#[doc = "Field `DBGTMP` reader - Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
pub type DBGTMP_R = crate::FieldReader<u32>;
#[doc = "Field `DBGTMP` writer - Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
pub type DBGTMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
    #[inline(always)]
    pub fn dbgtmp(&self) -> DBGTMP_R {
        DBGTMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
    #[inline(always)]
    #[must_use]
    pub fn dbgtmp(&mut self) -> DBGTMP_W<DCRDR_SPEC> {
        DBGTMP_W::new(self, 0)
    }
}
#[doc = "With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE  

You can [`read`](crate::Reg::read) this register and get [`dcrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCRDR_SPEC;
impl crate::RegisterSpec for DCRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrdr::R`](R) reader structure"]
impl crate::Readable for DCRDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcrdr::W`](W) writer structure"]
impl crate::Writable for DCRDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DCRDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
