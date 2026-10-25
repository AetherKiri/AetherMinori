# Recovered Snow transparent intermediate

Implemented isolated ef_snow_surface: zero-initialized canvas eachcall, clipped sprite positions, exactnative packedmaximum andeffectalpha. Tests verify repeatedoverlapdoesnotaccumulatealpha, transparentuntouchedpixels, negativepositionclip, empty/zeroalpha redrawclears. This composes only recovered intermediate; no finaleffect->sceneformula assumed, productionSnowstillnotwired. Suite136passed/15ignored plus1CLI,diffcheckclean. Native final-layer audit delegated separately. No new screenshot/fadeacceptance.
