#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<PIDR0_SPEC>;
#[doc = "Register `PIDR0` writer"]
pub type W = crate::W<PIDR0_SPEC>;
#[doc = "Field `PART_0` reader - Bits\\[7:0\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits\\[7:0\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "CoreSight Peripheral ID0  

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr0::W`](W) writer structure"]
impl crate::Writable for PIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR0 to value 0x21"]
impl crate::Resettable for PIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
