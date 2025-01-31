#[doc = "Register `pmusleep` writer"]
pub struct W(crate::W<PMUSLEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUSLEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMUSLEEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUSLEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sleep` writer - "]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Sleep Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmusleep](index.html) module"]
pub struct PMUSLEEP_SPEC;
impl crate::RegisterSpec for PMUSLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmusleep::W](W) writer structure"]
impl crate::Writable for PMUSLEEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmusleep to value 0"]
impl crate::Resettable for PMUSLEEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
