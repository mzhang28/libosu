import libosu

beatmap = libosu.parse_beatmap_file("tests/files/1360.osu")

print(repr(beatmap))
print(f"  Mode:       {beatmap.mode}")
print(f"  Audio:      {beatmap.audio_filename}")
print(f"  Countdown:  {beatmap.countdown}")

diff = beatmap.difficulty
print(f"\n{repr(diff)}")
print(f"  Circle radius (px): {diff.circle_size_osupx():.2f}")
print(f"  Approach preempt:   {diff.approach_preempt_ms()} ms")

objects = beatmap.hit_objects
print(f"\nHit objects: {len(objects)} total")
print(f"  Circles:  {beatmap.circle_count()}")
print(f"  Sliders:  {beatmap.slider_count()}")
print(f"  Spinners: {beatmap.spinner_count()}")

print(f"\nFirst 5 objects:")
for obj in objects[:5]:
    print(f"  {repr(obj)}")
    print(f"    additions={repr(obj.additions)}  sample={repr(obj.sample_info)}")
    if obj.is_slider():
        s = obj.slider_info()
        print(f"    {repr(s)}")
    if obj.is_spinner():
        print(f"    {repr(obj.spinner_info())}")

tps = beatmap.timing_points
print(f"\nTiming points: {len(tps)}")
for tp in tps:
    print(f"  {repr(tp)}")

print(f"\nCombo colors: {len(beatmap.colors)}")
for c in beatmap.colors:
    print(f"  {repr(c)}")

print(f"\nEvents: {len(beatmap.events)}")
for e in beatmap.events:
    print(f"  {repr(e)}")

print(f"\nMods examples:")
print(f"  HDDT = {repr(libosu.Mods.from_bits(8 | 64))}")
m = libosu.Mods.from_bits(8 | 64)
print(f"    hidden={m.hidden}  double_time={m.double_time}  hard_rock={m.hard_rock}")
