# Snow final-layer route: alpha-keyed masked copy

## Evidence chain

1. Blend decode (ef-re/snow-blend-equation.txt): 0x521b80 is an alpha-keyed whole-pixel copy (skip srcA==0, else overwrite the full dword including alpha); 0x46bec0 behind the 0x522d20 row-copy route is a raw memmove with no alpha test; the 0x523380 fade lerp requires image byte +0x18 == 0 and a full census found no writer ever storing 0 there, so the fade route is statically excluded. Snow code never writes +0x17/+0x18/+0x34; image defaults come from ctor 0x5245c0 (+0x17=0, +0x18=0x0F).
2. Corpus contradiction (decoded 000_01mugai.sc): the script stages WHITE at line1017, starts SnowH at line1019, then lines1022-1050 transition through several church scenery images (bgw_normal011a01_03*.png) while SnowH stays active. Route A (plain row copy) would overwrite every pixel with the zero-filled intermediate and destroy exactly those scenes, so the only self-consistent route is B: +0x17=1 via the shared activation pattern, drawing through 0x521b80.
3. Production change: composite_particles now rebuilds the zero-filled intermediate (ef_snow_surface, packed max with the fade alpha) and applies the masked copy over the composed scene. Source-over is retired; the six decoded blend modes stay as isolated units (commit e19dbad).

## Run and personally read frames

Rebuilt minori-headless; replayed 000_01mugai.sc English 60000 updates delta120 clicks600 every500; exit0, zero trace errors; message4250 at tick42309, 4251 at 42491, 4254 at 42739 (the SnowH-over-scenery window).

- frame-42500.png sha256 c4a1bc629d154608f44ef7a3d74ee7a8712f951ed4668a58b09747651fa9197f: personally read - church steeple scenery fully visible under active SnowH, hard-edged white flakes composited by the masked copy, cinematic black bands untouched. Route A would have blacked this frame out.

## Limits

- U4: whether the original final present resolves framebuffer alpha is unknown; this runtime keeps the scaled alpha in the output pixels exactly as the native route writes them, so fade frames carry partial transparency.
- The fade window itself was accepted in SNOW_FADE_INTEGRATION.md under the source-over composer; the same SnowFadeState drives this route (unit tests cover alpha 0/128/255 masked behavior).
- Sakura/Rain share the ParticleState path and therefore this route; their native image records were not independently traced.
- Static analysis only; no runtime oracle. U1-U5 remain listed in ef-re/snow-route-evidence.txt.
