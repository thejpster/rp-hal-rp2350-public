#[doc = "Register `BADPASSWD` reader"]
pub type R = crate::R<BADPASSWD_SPEC>;
#[doc = "Register `BADPASSWD` writer"]
pub type W = crate::W<BADPASSWD_SPEC>;
#[doc = "Field `BADPASSWD` reader - "]
pub type BADPASSWD_R = crate::BitReader;
#[doc = "Field `BADPASSWD` writer - "]
pub type BADPASSWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn badpasswd(&self) -> BADPASSWD_R {
        BADPASSWD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn badpasswd(&mut self) -> BADPASSWD_W<BADPASSWD_SPEC> {
        BADPASSWD_W::new(self, 0)
    }
}
#[doc = "Indicates a bad password has been used  

You can [`read`](crate::Reg::read) this register and get [`badpasswd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`badpasswd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BADPASSWD_SPEC;
impl crate::RegisterSpec for BADPASSWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`badpasswd::R`](R) reader structure"]
impl crate::Readable for BADPASSWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`badpasswd::W`](W) writer structure"]
impl crate::Writable for BADPASSWD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets BADPASSWD to value 0"]
impl crate::Resettable for BADPASSWD_SPEC {
    const RESET_VALUE: u32 = 0;
}
