#[doc = "Register `CLK_PERI_SELECTED` reader"]
pub type R = crate::R<CLK_PERI_SELECTED_SPEC>;
#[doc = "Field `CLK_PERI_SELECTED` reader - This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub type CLK_PERI_SELECTED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn clk_peri_selected(&self) -> CLK_PERI_SELECTED_R {
        CLK_PERI_SELECTED_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)  

You can [`read`](crate::Reg::read) this register and get [`clk_peri_selected::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PERI_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_PERI_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_peri_selected::R`](R) reader structure"]
impl crate::Readable for CLK_PERI_SELECTED_SPEC {}
#[doc = "`reset()` method sets CLK_PERI_SELECTED to value 0x01"]
impl crate::Resettable for CLK_PERI_SELECTED_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
