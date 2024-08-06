#[doc = "Register `ID_ISAR5` reader"]
pub type R = crate::R<ID_ISAR5_SPEC>;
#[doc = "Register `ID_ISAR5` writer"]
pub type W = crate::W<ID_ISAR5_SPEC>;
#[doc = "Field `ID_ISAR5` reader - "]
pub type ID_ISAR5_R = crate::FieldReader<u32>;
#[doc = "Field `ID_ISAR5` writer - "]
pub type ID_ISAR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn id_isar5(&self) -> ID_ISAR5_R {
        ID_ISAR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn id_isar5(&mut self) -> ID_ISAR5_W<ID_ISAR5_SPEC> {
        ID_ISAR5_W::new(self, 0)
    }
}
#[doc = "Provides information about the instruction set implemented by the PE  

You can [`read`](crate::Reg::read) this register and get [`id_isar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_isar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_ISAR5_SPEC;
impl crate::RegisterSpec for ID_ISAR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_isar5::R`](R) reader structure"]
impl crate::Readable for ID_ISAR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_isar5::W`](W) writer structure"]
impl crate::Writable for ID_ISAR5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_ISAR5 to value 0"]
impl crate::Resettable for ID_ISAR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
