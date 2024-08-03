#[doc = "Register `DPC` reader"]
pub type R = crate::R<DPC_SPEC>;
#[doc = "Register `DPC` writer"]
pub type W = crate::W<DPC_SPEC>;
#[doc = "Field `DPC` reader - "]
pub type DPC_R = crate::FieldReader<u32>;
#[doc = "Field `DPC` writer - "]
pub type DPC_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn dpc(&mut self) -> DPC_W<DPC_SPEC> {
        DPC_W::new(self, 1)
    }
}
#[doc = "Debug program counter. When entering Debug Mode, `dpc` samples the current program counter, e.g. the address of an `ebreak` which caused Debug Mode entry. When leaving debug mode, the processor jumps to `dpc`. The host may read/write this register whilst in Debug Mode.  

You can [`read`](crate::Reg::read) this register and get [`dpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPC_SPEC;
impl crate::RegisterSpec for DPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc::R`](R) reader structure"]
impl crate::Readable for DPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpc::W`](W) writer structure"]
impl crate::Writable for DPC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPC to value 0"]
impl crate::Resettable for DPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
