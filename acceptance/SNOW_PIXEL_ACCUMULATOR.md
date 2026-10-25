# Native Snow packed-pixel accumulator

Independently read raw522c81..99 and522cdb..e9 in ef-re/snow-fadeout-native.txt: scaled sourcealpha lookup, keepRGB, compare unsigned packedAARRGGBB source>destination, replacewholepixel. This is NOT component-wise max nor sourceover. Added isolated ef_snow_max_pixel RGBA adapter with255alpha scalingdomain, alpha-priority/RGBtie-order and negativealpha fixtures. Suite131passed/14ignored plus1CLI,diffcheckclean.

Production not wired: drawing packedmax directly onto opaque scene would suppress most flakes (destalpha255 dominates); original uses effect-owned backing surface, which must be reconstructed first. Native clock mapping also under audit. No screenshot/newfidelityclaim.

Terminal audit separately confirms dirty-store serialization exists in native destructors; absenceEndflush not proof original losesglobals. Providerteardown enhancement delegated with reopeningtest, not immediateEndrewrite.
