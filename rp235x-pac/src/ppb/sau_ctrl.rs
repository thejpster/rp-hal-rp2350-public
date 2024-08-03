#[doc = "Register `SAU_CTRL` reader"]
pub type R = crate::R<SAU_CTRL_SPEC>;
#[doc = "Register `SAU_CTRL` writer"]
pub type W = crate::W<SAU_CTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Enables the SAU"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the SAU"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALLNS` reader - When SAU_CTRL.ENABLE is 0 this bit controls if the memory is marked as Non-secure or Secure"]
pub type ALLNS_R = crate::BitReader;
#[doc = "Field `ALLNS` writer - When SAU_CTRL.ENABLE is 0 this bit controls if the memory is marked as Non-secure or Secure"]
pub type ALLNS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the SAU"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When SAU_CTRL.ENABLE is 0 this bit controls if the memory is marked as Non-secure or Secure"]
    #[inline(always)]
    pub fn allns(&self) -> ALLNS_R {
        ALLNS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the SAU"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<SAU_CTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - When SAU_CTRL.ENABLE is 0 this bit controls if the memory is marked as Non-secure or Secure"]
    #[inline(always)]
    #[must_use]
    pub fn allns(&mut self) -> ALLNS_W<SAU_CTRL_SPEC> {
        ALLNS_W::new(self, 1)
    }
}
#[doc = "Allows enabling of the Security Attribution Unit  

You can [`read`](crate::Reg::read) this register and get [`sau_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sau_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAU_CTRL_SPEC;
impl crate::RegisterSpec for SAU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sau_ctrl::R`](R) reader structure"]
impl crate::Readable for SAU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sau_ctrl::W`](W) writer structure"]
impl crate::Writable for SAU_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAU_CTRL to value 0"]
impl crate::Resettable for SAU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
