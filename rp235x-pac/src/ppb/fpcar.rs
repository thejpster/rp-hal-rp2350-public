#[doc = "Register `FPCAR` reader"]
pub type R = crate::R<FPCAR_SPEC>;
#[doc = "Register `FPCAR` writer"]
pub type W = crate::W<FPCAR_SPEC>;
#[doc = "Field `ADDRESS` reader - The location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - The location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - The location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - The location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<FPCAR_SPEC> {
        ADDRESS_W::new(self, 3)
    }
}
#[doc = "Holds the location of the unpopulated floating-point register space allocated on an exception stack frame  

You can [`read`](crate::Reg::read) this register and get [`fpcar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpcar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCAR_SPEC;
impl crate::RegisterSpec for FPCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpcar::R`](R) reader structure"]
impl crate::Readable for FPCAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpcar::W`](W) writer structure"]
impl crate::Writable for FPCAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCAR to value 0"]
impl crate::Resettable for FPCAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
