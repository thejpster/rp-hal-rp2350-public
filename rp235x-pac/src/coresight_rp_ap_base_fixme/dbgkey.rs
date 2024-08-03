#[doc = "Register `DBGKEY` reader"]
pub type R = crate::R<DBGKEY_SPEC>;
#[doc = "Register `DBGKEY` writer"]
pub type W = crate::W<DBGKEY_SPEC>;
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::BitReader;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSH` reader - "]
pub type PUSH_R = crate::BitReader;
#[doc = "Field `PUSH` writer - "]
pub type PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset (before sending a new key)"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Reset (before sending a new key)"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn push(&self) -> PUSH_R {
        PUSH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset (before sending a new key)"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DBGKEY_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn push(&mut self) -> PUSH_W<DBGKEY_SPEC> {
        PUSH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset (before sending a new key)"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<DBGKEY_SPEC> {
        RESET_W::new(self, 2)
    }
}
#[doc = "Serial key load interface (write-only)  

You can [`read`](crate::Reg::read) this register and get [`dbgkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGKEY_SPEC;
impl crate::RegisterSpec for DBGKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgkey::R`](R) reader structure"]
impl crate::Readable for DBGKEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgkey::W`](W) writer structure"]
impl crate::Writable for DBGKEY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGKEY to value 0"]
impl crate::Resettable for DBGKEY_SPEC {
    const RESET_VALUE: u32 = 0;
}
