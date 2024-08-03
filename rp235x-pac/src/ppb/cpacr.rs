#[doc = "Register `CPACR` reader"]
pub type R = crate::R<CPACR_SPEC>;
#[doc = "Register `CPACR` writer"]
pub type W = crate::W<CPACR_SPEC>;
#[doc = "Field `CP0` reader - Controls access privileges for coprocessor 0"]
pub type CP0_R = crate::FieldReader;
#[doc = "Field `CP0` writer - Controls access privileges for coprocessor 0"]
pub type CP0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP1` reader - Controls access privileges for coprocessor 1"]
pub type CP1_R = crate::FieldReader;
#[doc = "Field `CP1` writer - Controls access privileges for coprocessor 1"]
pub type CP1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP2` reader - Controls access privileges for coprocessor 2"]
pub type CP2_R = crate::FieldReader;
#[doc = "Field `CP2` writer - Controls access privileges for coprocessor 2"]
pub type CP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP3` reader - Controls access privileges for coprocessor 3"]
pub type CP3_R = crate::FieldReader;
#[doc = "Field `CP3` writer - Controls access privileges for coprocessor 3"]
pub type CP3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP4` reader - Controls access privileges for coprocessor 4"]
pub type CP4_R = crate::FieldReader;
#[doc = "Field `CP4` writer - Controls access privileges for coprocessor 4"]
pub type CP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP5` reader - Controls access privileges for coprocessor 5"]
pub type CP5_R = crate::FieldReader;
#[doc = "Field `CP5` writer - Controls access privileges for coprocessor 5"]
pub type CP5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP6` reader - Controls access privileges for coprocessor 6"]
pub type CP6_R = crate::FieldReader;
#[doc = "Field `CP6` writer - Controls access privileges for coprocessor 6"]
pub type CP6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP7` reader - Controls access privileges for coprocessor 7"]
pub type CP7_R = crate::FieldReader;
#[doc = "Field `CP7` writer - Controls access privileges for coprocessor 7"]
pub type CP7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP10` reader - Defines the access rights for the floating-point functionality"]
pub type CP10_R = crate::FieldReader;
#[doc = "Field `CP10` writer - Defines the access rights for the floating-point functionality"]
pub type CP10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CP11` reader - The value in this field is ignored. If the implementation does not include the FP Extension, this field is RAZ/WI. If the value of this bit is not programmed to the same value as the CP10 field, then the value is UNKNOWN"]
pub type CP11_R = crate::FieldReader;
#[doc = "Field `CP11` writer - The value in this field is ignored. If the implementation does not include the FP Extension, this field is RAZ/WI. If the value of this bit is not programmed to the same value as the CP10 field, then the value is UNKNOWN"]
pub type CP11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Controls access privileges for coprocessor 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Controls access privileges for coprocessor 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Controls access privileges for coprocessor 2"]
    #[inline(always)]
    pub fn cp2(&self) -> CP2_R {
        CP2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Controls access privileges for coprocessor 3"]
    #[inline(always)]
    pub fn cp3(&self) -> CP3_R {
        CP3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Controls access privileges for coprocessor 4"]
    #[inline(always)]
    pub fn cp4(&self) -> CP4_R {
        CP4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Controls access privileges for coprocessor 5"]
    #[inline(always)]
    pub fn cp5(&self) -> CP5_R {
        CP5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Controls access privileges for coprocessor 6"]
    #[inline(always)]
    pub fn cp6(&self) -> CP6_R {
        CP6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Controls access privileges for coprocessor 7"]
    #[inline(always)]
    pub fn cp7(&self) -> CP7_R {
        CP7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Defines the access rights for the floating-point functionality"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The value in this field is ignored. If the implementation does not include the FP Extension, this field is RAZ/WI. If the value of this bit is not programmed to the same value as the CP10 field, then the value is UNKNOWN"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls access privileges for coprocessor 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<CPACR_SPEC> {
        CP0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Controls access privileges for coprocessor 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<CPACR_SPEC> {
        CP1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Controls access privileges for coprocessor 2"]
    #[inline(always)]
    #[must_use]
    pub fn cp2(&mut self) -> CP2_W<CPACR_SPEC> {
        CP2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Controls access privileges for coprocessor 3"]
    #[inline(always)]
    #[must_use]
    pub fn cp3(&mut self) -> CP3_W<CPACR_SPEC> {
        CP3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Controls access privileges for coprocessor 4"]
    #[inline(always)]
    #[must_use]
    pub fn cp4(&mut self) -> CP4_W<CPACR_SPEC> {
        CP4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Controls access privileges for coprocessor 5"]
    #[inline(always)]
    #[must_use]
    pub fn cp5(&mut self) -> CP5_W<CPACR_SPEC> {
        CP5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Controls access privileges for coprocessor 6"]
    #[inline(always)]
    #[must_use]
    pub fn cp6(&mut self) -> CP6_W<CPACR_SPEC> {
        CP6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Controls access privileges for coprocessor 7"]
    #[inline(always)]
    #[must_use]
    pub fn cp7(&mut self) -> CP7_W<CPACR_SPEC> {
        CP7_W::new(self, 14)
    }
    #[doc = "Bits 20:21 - Defines the access rights for the floating-point functionality"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<CPACR_SPEC> {
        CP10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - The value in this field is ignored. If the implementation does not include the FP Extension, this field is RAZ/WI. If the value of this bit is not programmed to the same value as the CP10 field, then the value is UNKNOWN"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<CPACR_SPEC> {
        CP11_W::new(self, 22)
    }
}
#[doc = "Specifies the access privileges for coprocessors and the FP Extension  

You can [`read`](crate::Reg::read) this register and get [`cpacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpacr::R`](R) reader structure"]
impl crate::Readable for CPACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpacr::W`](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
