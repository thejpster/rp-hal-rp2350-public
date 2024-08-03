#[doc = "Register `BOOTLOCK5` reader"]
pub type R = crate::R<BOOTLOCK5_SPEC>;
#[doc = "Register `BOOTLOCK5` writer"]
pub type W = crate::W<BOOTLOCK5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTLOCK5_SPEC;
impl crate::RegisterSpec for BOOTLOCK5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootlock5::R`](R) reader structure"]
impl crate::Readable for BOOTLOCK5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootlock5::W`](W) writer structure"]
impl crate::Writable for BOOTLOCK5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTLOCK5 to value 0"]
impl crate::Resettable for BOOTLOCK5_SPEC {
    const RESET_VALUE: u32 = 0;
}
