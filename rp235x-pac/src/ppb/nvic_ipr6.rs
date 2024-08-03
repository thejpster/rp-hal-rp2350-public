#[doc = "Register `NVIC_IPR6` reader"]
pub type R = crate::R<NVIC_IPR6_SPEC>;
#[doc = "Register `NVIC_IPR6` writer"]
pub type W = crate::W<NVIC_IPR6_SPEC>;
#[doc = "Field `PRI_N0` reader - For register NVIC_IPRn, the priority of interrupt number 4*n+0, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N0_R = crate::FieldReader;
#[doc = "Field `PRI_N0` writer - For register NVIC_IPRn, the priority of interrupt number 4*n+0, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI_N1` reader - For register NVIC_IPRn, the priority of interrupt number 4*n+1, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N1_R = crate::FieldReader;
#[doc = "Field `PRI_N1` writer - For register NVIC_IPRn, the priority of interrupt number 4*n+1, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI_N2` reader - For register NVIC_IPRn, the priority of interrupt number 4*n+2, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N2_R = crate::FieldReader;
#[doc = "Field `PRI_N2` writer - For register NVIC_IPRn, the priority of interrupt number 4*n+2, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI_N3` reader - For register NVIC_IPRn, the priority of interrupt number 4*n+3, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N3_R = crate::FieldReader;
#[doc = "Field `PRI_N3` writer - For register NVIC_IPRn, the priority of interrupt number 4*n+3, or RES0 if the PE does not implement this interrupt"]
pub type PRI_N3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - For register NVIC_IPRn, the priority of interrupt number 4*n+0, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n0(&self) -> PRI_N0_R {
        PRI_N0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - For register NVIC_IPRn, the priority of interrupt number 4*n+1, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n1(&self) -> PRI_N1_R {
        PRI_N1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - For register NVIC_IPRn, the priority of interrupt number 4*n+2, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n2(&self) -> PRI_N2_R {
        PRI_N2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - For register NVIC_IPRn, the priority of interrupt number 4*n+3, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    pub fn pri_n3(&self) -> PRI_N3_R {
        PRI_N3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - For register NVIC_IPRn, the priority of interrupt number 4*n+0, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n0(&mut self) -> PRI_N0_W<NVIC_IPR6_SPEC> {
        PRI_N0_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - For register NVIC_IPRn, the priority of interrupt number 4*n+1, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n1(&mut self) -> PRI_N1_W<NVIC_IPR6_SPEC> {
        PRI_N1_W::new(self, 12)
    }
    #[doc = "Bits 20:23 - For register NVIC_IPRn, the priority of interrupt number 4*n+2, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n2(&mut self) -> PRI_N2_W<NVIC_IPR6_SPEC> {
        PRI_N2_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - For register NVIC_IPRn, the priority of interrupt number 4*n+3, or RES0 if the PE does not implement this interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_n3(&mut self) -> PRI_N3_W<NVIC_IPR6_SPEC> {
        PRI_N3_W::new(self, 28)
    }
}
#[doc = "Sets or reads interrupt priorities  

You can [`read`](crate::Reg::read) this register and get [`nvic_ipr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_ipr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR6_SPEC;
impl crate::RegisterSpec for NVIC_IPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr6::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr6::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR6 to value 0"]
impl crate::Resettable for NVIC_IPR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
