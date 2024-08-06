#[doc = "Register `ID_MMFR0` reader"]
pub type R = crate::R<ID_MMFR0_SPEC>;
#[doc = "Register `ID_MMFR0` writer"]
pub type W = crate::W<ID_MMFR0_SPEC>;
#[doc = "Field `PMSA` reader - Indicates support for the protected memory system architecture (PMSA)"]
pub type PMSA_R = crate::FieldReader;
#[doc = "Field `OUTERSHR` reader - Indicates the outermost shareability domain implemented"]
pub type OUTERSHR_R = crate::FieldReader;
#[doc = "Field `SHARELVL` reader - Indicates the number of shareability levels implemented"]
pub type SHARELVL_R = crate::FieldReader;
#[doc = "Field `TCM` reader - Indicates support for tightly coupled memories (TCMs)"]
pub type TCM_R = crate::FieldReader;
#[doc = "Field `AUXREG` reader - Indicates support for Auxiliary Control Registers"]
pub type AUXREG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Indicates support for the protected memory system architecture (PMSA)"]
    #[inline(always)]
    pub fn pmsa(&self) -> PMSA_R {
        PMSA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the outermost shareability domain implemented"]
    #[inline(always)]
    pub fn outershr(&self) -> OUTERSHR_R {
        OUTERSHR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the number of shareability levels implemented"]
    #[inline(always)]
    pub fn sharelvl(&self) -> SHARELVL_R {
        SHARELVL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates support for tightly coupled memories (TCMs)"]
    #[inline(always)]
    pub fn tcm(&self) -> TCM_R {
        TCM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates support for Auxiliary Control Registers"]
    #[inline(always)]
    pub fn auxreg(&self) -> AUXREG_R {
        AUXREG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_MMFR0_SPEC;
impl crate::RegisterSpec for ID_MMFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr0::R`](R) reader structure"]
impl crate::Readable for ID_MMFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_mmfr0::W`](W) writer structure"]
impl crate::Writable for ID_MMFR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_MMFR0 to value 0x0010_1f40"]
impl crate::Resettable for ID_MMFR0_SPEC {
    const RESET_VALUE: u32 = 0x0010_1f40;
}
