# Destroy-only global durability acceptance

Providerdestroy now best-effort saves changedglobals beforemedia/drop iffVMpresent andwritablepathnonempty. Existingserializer createsmissingdirs; unchanged/noVM/null cases do notwrite. Persistencefailuredoesnot preventcleanup. Scope matches observed native destructor dirtysave boundary but doesNOT assert retailformat/fullnativepolicy. End still no immediateflush; replacingVM viaopen_game unchanged. Globalsonly, notreadhistory.

Regression runsBADEND1assignment,end,EOF then ABI destroy; reads saved system.dat andfreshVM. Opt-in actuallyopen_game reloads samewritabledirectory andseesBADEND1=1. Failurefixtureusesdirectoryat system.dat toensurewritefailure andassertsmediaremoved. Parentreviewed source andindependentlyran135lib+1CLIdefault,15optins passed4.15s,diffcheckclean. No screenshotneededforstatepersistence. Snowclockadapter separatecommit.
