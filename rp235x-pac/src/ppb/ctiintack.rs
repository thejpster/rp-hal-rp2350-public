#[doc = "Register `CTIINTACK` reader"]
pub type R = crate::R<CTIINTACK_SPEC>;
#[doc = "Register `CTIINTACK` writer"]
pub type W = crate::W<CTIINTACK_SPEC>;
#[doc = "Field `INTACK` reader - Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged, causing it to be cleared."]
pub type INTACK_R = crate::FieldReader;
#[doc = "Field `INTACK` writer - Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged, causing it to be cleared."]
pub type INTACK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged, causing it to be cleared."]
    #[inline(always)]
    pub fn intack(&self) -> INTACK_R {
        INTACK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged, causing it to be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intack(&mut self) -> INTACK_W<CTIINTACK_SPEC> {
        INTACK_W::new(self, 0)
    }
}
#[doc = "CTI Interrupt Acknowledge Register  

You can [`read`](crate::Reg::read) this register and get [`ctiintack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiintack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIINTACK_SPEC;
impl crate::RegisterSpec for CTIINTACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiintack::R`](R) reader structure"]
impl crate::Readable for CTIINTACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctiintack::W`](W) writer structure"]
impl crate::Writable for CTIINTACK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIINTACK to value 0"]
impl crate::Resettable for CTIINTACK_SPEC {
    const RESET_VALUE: u32 = 0;
}
