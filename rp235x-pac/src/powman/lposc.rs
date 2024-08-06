#[doc = "Register `LPOSC` reader"]
pub type R = crate::R<LPOSC_SPEC>;
#[doc = "Register `LPOSC` writer"]
pub type W = crate::W<LPOSC_SPEC>;
#[doc = "Field `MODE` reader - This feature has been removed"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - This feature has been removed"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIM` reader - Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - This feature has been removed"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:9 - Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This feature has been removed"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<LPOSC_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:9 - Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<LPOSC_SPEC> {
        TRIM_W::new(self, 4)
    }
}
#[doc = "Low power oscillator control register.  

You can [`read`](crate::Reg::read) this register and get [`lposc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOSC_SPEC;
impl crate::RegisterSpec for LPOSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposc::R`](R) reader structure"]
impl crate::Readable for LPOSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lposc::W`](W) writer structure"]
impl crate::Writable for LPOSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOSC to value 0x0203"]
impl crate::Resettable for LPOSC_SPEC {
    const RESET_VALUE: u32 = 0x0203;
}
