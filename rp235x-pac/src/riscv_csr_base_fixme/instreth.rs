#[doc = "Register `INSTRETH` reader"]
pub type R = crate::R<INSTRETH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read-only U-mode alias of minstreth, accessible when `mcounteren.ir` is set  

You can [`read`](crate::Reg::read) this register and get [`instreth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTRETH_SPEC;
impl crate::RegisterSpec for INSTRETH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instreth::R`](R) reader structure"]
impl crate::Readable for INSTRETH_SPEC {}
#[doc = "`reset()` method sets INSTRETH to value 0"]
impl crate::Resettable for INSTRETH_SPEC {
    const RESET_VALUE: u32 = 0;
}
