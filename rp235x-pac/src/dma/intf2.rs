#[doc = "Register `INTF2` reader"]
pub type R = crate::R<INTF2_SPEC>;
#[doc = "Register `INTF2` writer"]
pub type W = crate::W<INTF2_SPEC>;
#[doc = "Field `INTF2` reader - Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
pub type INTF2_R = crate::FieldReader<u16>;
#[doc = "Field `INTF2` writer - Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
pub type INTF2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
    #[inline(always)]
    pub fn intf2(&self) -> INTF2_R {
        INTF2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intf2(&mut self) -> INTF2_W<INTF2_SPEC> {
        INTF2_W::new(self, 0)
    }
}
#[doc = "Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF2_SPEC;
impl crate::RegisterSpec for INTF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf2::R`](R) reader structure"]
impl crate::Readable for INTF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf2::W`](W) writer structure"]
impl crate::Writable for INTF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF2 to value 0"]
impl crate::Resettable for INTF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
