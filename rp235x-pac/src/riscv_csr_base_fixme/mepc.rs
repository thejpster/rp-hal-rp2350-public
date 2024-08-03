#[doc = "Register `MEPC` reader"]
pub type R = crate::R<MEPC_SPEC>;
#[doc = "Register `MEPC` writer"]
pub type W = crate::W<MEPC_SPEC>;
#[doc = "Field `MEPC` reader - "]
pub type MEPC_R = crate::FieldReader<u32>;
#[doc = "Field `MEPC` writer - "]
pub type MEPC_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn mepc(&self) -> MEPC_R {
        MEPC_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    #[must_use]
    pub fn mepc(&mut self) -> MEPC_W<MEPC_SPEC> {
        MEPC_W::new(self, 2)
    }
}
#[doc = "Machine exception program counter.  

 When entering a trap, the current value of the program counter is recorded here. When executing an `mret`, the processor jumps to `mepc`. Can also be read and written by software.  

You can [`read`](crate::Reg::read) this register and get [`mepc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mepc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEPC_SPEC;
impl crate::RegisterSpec for MEPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mepc::R`](R) reader structure"]
impl crate::Readable for MEPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mepc::W`](W) writer structure"]
impl crate::Writable for MEPC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEPC to value 0"]
impl crate::Resettable for MEPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
