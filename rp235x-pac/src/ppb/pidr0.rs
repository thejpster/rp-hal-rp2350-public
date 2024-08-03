#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<PIDR0_SPEC>;
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
#[doc = "CoreSight Periperal ID0  

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {}
#[doc = "`reset()` method sets PIDR0 to value 0x21"]
impl crate::Resettable for PIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
