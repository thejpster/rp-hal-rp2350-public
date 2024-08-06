#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<PIDR1_SPEC>;
#[doc = "Register `PIDR1` writer"]
pub type W = crate::W<PIDR1_SPEC>;
#[doc = "Field `PART_1` reader - Bits\\[11:8\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub type PART_1_R = crate::FieldReader;
#[doc = "Field `DES_0` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type DES_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Bits\\[11:8\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "CoreSight Peripheral ID1  

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr1::W`](W) writer structure"]
impl crate::Writable for PIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR1 to value 0xbd"]
impl crate::Resettable for PIDR1_SPEC {
    const RESET_VALUE: u32 = 0xbd;
}
