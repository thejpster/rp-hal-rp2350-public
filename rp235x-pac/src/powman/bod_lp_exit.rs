#[doc = "Register `BOD_LP_EXIT` reader"]
pub type R = crate::R<BOD_LP_EXIT_SPEC>;
#[doc = "Register `BOD_LP_EXIT` writer"]
pub type W = crate::W<BOD_LP_EXIT_SPEC>;
#[doc = "Field `EN` reader - enable brown-out detection 0=not enabled, 1=enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable brown-out detection 0=not enabled, 1=enabled"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEL` reader - threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
pub type VSEL_R = crate::FieldReader;
#[doc = "Field `VSEL` writer - threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
pub type VSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BOD_LP_EXIT_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<BOD_LP_EXIT_SPEC> {
        VSEL_W::new(self, 4)
    }
}
#[doc = "Brown-out Detection Low Power Exit Settings  

You can [`read`](crate::Reg::read) this register and get [`bod_lp_exit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod_lp_exit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOD_LP_EXIT_SPEC;
impl crate::RegisterSpec for BOD_LP_EXIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod_lp_exit::R`](R) reader structure"]
impl crate::Readable for BOD_LP_EXIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bod_lp_exit::W`](W) writer structure"]
impl crate::Writable for BOD_LP_EXIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOD_LP_EXIT to value 0xb1"]
impl crate::Resettable for BOD_LP_EXIT_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
