#[doc = "Register `CRT_KEY_W1` writer"]
pub type W = crate::W<CRT_KEY_W1_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<CRT_KEY_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Word 1 (bits 63..32) of the key. Write only, read returns 0x0  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crt_key_w1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRT_KEY_W1_SPEC;
impl crate::RegisterSpec for CRT_KEY_W1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crt_key_w1::W`](W) writer structure"]
impl crate::Writable for CRT_KEY_W1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRT_KEY_W1 to value 0"]
impl crate::Resettable for CRT_KEY_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
