# Native Snow/SnowH fade-state port

Read raw50b920 trigger and50bc70..95 accumulator/update plus50bdcf signedcompletion in ef-re/snow-fadeout-native.txt. Isolated SnowFadeState startsalpha0, idempotentfade resets elapsedonlyfirsttime, accepted delta cumulatively addedu32 then q=elapsed>>4 eachupdate; normalalpha unsignedadd cap256,fade signedwrappingsub withoutzeroclamp,doneiffading&&alpha<1. Testscover1/3/6/10 ramp,cap256,repeatedtrigger retains elapsed,negativecompletion,no updatesafterdone.

Suite130passed/14ignored plus1CLI,diffcheckclean. Production particle path NOT wired yet: native acceptedclockunits/cadence/dynamicminimumdelta unproved. Native alphahelper uses packedunsignedmax rather than current sourceover, and50particles vs current160/220. These are concrete nextintegration obligations; do not substitute immediateclear or claim350ms exact disappearance. No new screenshots.
