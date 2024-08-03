#[doc = "Register `INTF3` reader"]
pub type R = crate::R<INTF3_SPEC>;
#[doc = "Register `INTF3` writer"]
pub type W = crate::W<INTF3_SPEC>;
#[doc = "Field `INTF3` reader - Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
pub type INTF3_R = crate::FieldReader<u16>;
#[doc = "Field `INTF3` writer - Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
pub type INTF3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
    #[inline(always)]
    pub fn intf3(&self) -> INTF3_R {
        INTF3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intf3(&mut self) -> INTF3_W<INTF3_SPEC> {
        INTF3_W::new(self, 0)
    }
}
#[doc = "Force Interrupts  

You can [`read`](crate::Reg::read) this register and get [`intf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF3_SPEC;
impl crate::RegisterSpec for INTF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf3::R`](R) reader structure"]
impl crate::Readable for INTF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intf3::W`](W) writer structure"]
impl crate::Writable for INTF3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF3 to value 0"]
impl crate::Resettable for INTF3_SPEC {
    const RESET_VALUE: u32 = 0;
}
