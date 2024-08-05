#[doc = "Register `ID_PFR0` reader"]
pub type R = crate::R<ID_PFR0_SPEC>;
#[doc = "Register `ID_PFR0` writer"]
pub type W = crate::W<ID_PFR0_SPEC>;
#[doc = "Field `STATE0` reader - A32 instruction set support"]
pub type STATE0_R = crate::FieldReader;
#[doc = "Field `STATE1` reader - T32 instruction set support"]
pub type STATE1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - A32 instruction set support"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - T32 instruction set support"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Gives top-level information about the instruction set supported by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_pfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_pfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_PFR0_SPEC;
impl crate::RegisterSpec for ID_PFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_pfr0::R`](R) reader structure"]
impl crate::Readable for ID_PFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_pfr0::W`](W) writer structure"]
impl crate::Writable for ID_PFR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_PFR0 to value 0x30"]
impl crate::Resettable for ID_PFR0_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
