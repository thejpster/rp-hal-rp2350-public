#[doc = "Register `TSELECT` reader"]
pub type R = crate::R<TSELECT_SPEC>;
#[doc = "Register `TSELECT` writer"]
pub type W = crate::W<TSELECT_SPEC>;
#[doc = "Field `TSELECT` reader - "]
pub type TSELECT_R = crate::FieldReader;
#[doc = "Field `TSELECT` writer - "]
pub type TSELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tselect(&self) -> TSELECT_R {
        TSELECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tselect(&mut self) -> TSELECT_W<TSELECT_SPEC> {
        TSELECT_W::new(self, 0)
    }
}
#[doc = "Select trigger to be configured via `tdata1`/`tdata2`  

 On RP2350, four instruction address triggers are implemented, so only the two LSBs of this register are writable.  

You can [`read`](crate::Reg::read) this register and get [`tselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSELECT_SPEC;
impl crate::RegisterSpec for TSELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tselect::R`](R) reader structure"]
impl crate::Readable for TSELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tselect::W`](W) writer structure"]
impl crate::Writable for TSELECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSELECT to value 0"]
impl crate::Resettable for TSELECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
