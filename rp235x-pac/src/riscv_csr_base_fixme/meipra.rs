#[doc = "Register `MEIPRA` reader"]
pub type R = crate::R<MEIPRA_SPEC>;
#[doc = "Register `MEIPRA` writer"]
pub type W = crate::W<MEIPRA_SPEC>;
#[doc = "Field `INDEX` writer - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WINDOW` reader - 16-bit read/write window into the external interrupt priority array, containing four 4-bit priority values."]
pub type WINDOW_R = crate::FieldReader<u16>;
#[doc = "Field `WINDOW` writer - 16-bit read/write window into the external interrupt priority array, containing four 4-bit priority values."]
pub type WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt priority array, containing four 4-bit priority values."]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<MEIPRA_SPEC> {
        INDEX_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt priority array, containing four 4-bit priority values."]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<MEIPRA_SPEC> {
        WINDOW_W::new(self, 16)
    }
}
#[doc = "External interrupt priority array  

 Each interrupt has an (up to) 4-bit priority value associated with it, and each access to this register reads and/or writes a 16-bit window containing four such priority values. When less than 16 priority levels are available, the LSBs of the priority fields are hardwired to 0.  

 When an interrupt's priority is lower than the current preemption priority `meicontext.preempt`, it is treated as not being pending for the purposes of `mip.meip`. The pending bit in `meipa` will still assert, but the machine external interrupt pending bit `mip.meip` will not, so the processor will ignore this interrupt. See `meicontext`.  

You can [`read`](crate::Reg::read) this register and get [`meipra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meipra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEIPRA_SPEC;
impl crate::RegisterSpec for MEIPRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meipra::R`](R) reader structure"]
impl crate::Readable for MEIPRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meipra::W`](W) writer structure"]
impl crate::Writable for MEIPRA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEIPRA to value 0"]
impl crate::Resettable for MEIPRA_SPEC {
    const RESET_VALUE: u32 = 0;
}
