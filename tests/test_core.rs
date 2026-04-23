use std::fs::File;
use std::str::FromStr;

use anyhow::Result;
use libosu::prelude::*;

#[test]
fn test_difficulty_formulas() {
  let d = |ar: f32, cs: f32| Difficulty {
    approach_rate: ar,
    circle_size: cs,
    overall_difficulty: 5.0,
    hp_drain_rate: 5.0,
    slider_multiplier: 1.4,
    slider_tick_rate: 1.0,
  };

  // circle_size_osupx: 54.4 - 4.48 * cs
  assert!((d(5.0, 0.0).circle_size_osupx() - 54.4).abs() < 0.01);
  assert!((d(5.0, 5.0).circle_size_osupx() - 32.0).abs() < 0.01); // 54.4 - 22.4
  assert!((d(5.0, 10.0).circle_size_osupx() - 9.6).abs() < 0.01); // 54.4 - 44.8

  // approach_preempt, AR < 5: 1200 + (600*(5-AR)) / 5
  assert_eq!(d(0.0, 5.0).approach_preempt(), Millis(1800)); // 1200 + 600
  assert_eq!(d(4.0, 5.0).approach_preempt(), Millis(1320)); // 1200 + 120
                                                            // AR = 5: exactly 1200
  assert_eq!(d(5.0, 5.0).approach_preempt(), Millis(1200));
  // AR > 5: 1200 - (750*(AR-5)) / 5
  assert_eq!(d(10.0, 5.0).approach_preempt(), Millis(450)); // 1200 - 750

  // approach_fade_time, AR < 5: 800 + (400*(5-AR)) / 5
  assert_eq!(d(0.0, 5.0).approach_fade_time(), Millis(1200)); // 800 + 400
  assert_eq!(d(5.0, 5.0).approach_fade_time(), Millis(800));
  // AR > 5: 800 - (500*(AR-5)) / 5
  assert_eq!(d(10.0, 5.0).approach_fade_time(), Millis(300)); // 800 - 500
}

#[test]
fn test_mods_parse() {
  // All standard mods via comma delimiter
  let mods = Mods::parse_from_str("+NF,HR,SD,RX,HT,NC,FL,AU,SO,AP,PF", ",")
    .expect("should parse");
  assert!(mods.contains(Mods::NoFail));
  assert!(mods.contains(Mods::HardRock));
  assert!(mods.contains(Mods::SuddenDeath));
  assert!(mods.contains(Mods::Relax));
  assert!(mods.contains(Mods::HalfTime));
  assert!(mods.contains(Mods::Nightcore));
  assert!(mods.contains(Mods::Flashlight));
  assert!(mods.contains(Mods::Autoplay));
  assert!(mods.contains(Mods::SpunOut));
  assert!(mods.contains(Mods::Relax2));
  assert!(mods.contains(Mods::Perfect));

  // Mania key mods
  let key_mods = Mods::parse_from_str("+5K,6K,7K,8K,9K,1K,2K,3K,CM,TP", ",")
    .expect("key mods");
  assert!(key_mods.contains(Mods::Key5));
  assert!(key_mods.contains(Mods::Key6));
  assert!(key_mods.contains(Mods::Key7));
  assert!(key_mods.contains(Mods::Key8));
  assert!(key_mods.contains(Mods::Key9));
  assert!(key_mods.contains(Mods::Key1));
  assert!(key_mods.contains(Mods::Key2));
  assert!(key_mods.contains(Mods::Key3));
  assert!(key_mods.contains(Mods::LastMod));
  assert!(key_mods.contains(Mods::TargetPractice));

  // Wrong delimiter must return None (kills delimiter guard mutant)
  assert_eq!(Mods::parse_from_str("+NF,HR", "|"), None);

  // TouchDevice aliases
  assert_eq!(
    Mods::parse_from_str("TD", ""),
    Mods::parse_from_str("NV", "")
  );
}

#[test]
fn test_hitobject_predicates_and_parsing() -> Result<()> {
  // based on the file
  let file = File::open("tests/files/1360.osu")?;
  let beatmap = Beatmap::parse(file)?;
  let ho = &beatmap.hit_objects;

  assert!(ho[0].kind.is_circle());
  assert!(!ho[0].kind.is_slider());
  assert!(!ho[0].kind.is_spinner());
  assert_eq!(ho[0].start_time, Millis(7735));
  assert!(!ho[0].new_combo);

  assert!(ho[6].kind.is_circle());
  assert!(ho[6].new_combo);

  assert!(ho[5].kind.is_slider());
  assert!(!ho[5].kind.is_circle());
  assert!(!ho[5].kind.is_spinner());

  assert!(ho[19].kind.is_spinner());
  assert!(!ho[19].kind.is_circle());
  assert!(!ho[19].kind.is_slider());
  assert!(ho[19].new_combo);

  if let HitObjectKind::Spinner(ref info) = ho[19].kind {
    assert_eq!(info.end_time, Millis(26921));
  } else {
    panic!("expected spinner");
  }

  Ok(())
}

#[test]
fn test_hitobject_equality_and_ordering() -> Result<()> {
  // eq is based solely on start_time
  let a = HitObject::from_str("100,100,1000,1,0")?;
  let b = HitObject::from_str("200,200,1000,1,0")?; // same time, different pos
  let c = HitObject::from_str("100,100,2000,1,0")?; // different time

  assert_eq!(a, b);
  assert_ne!(a, c);

  // ordering by start_time
  assert!(a < c);
  assert!(c > a);
  assert!(!(a < b)); // equal times

  Ok(())
}

#[test]
fn test_beatmap_locate() -> Result<()> {
  let file = File::open("tests/files/1360.osu")?;
  let beatmap = Beatmap::parse(file)?;

  // locate_timing_point: returns the last tp strictly before the given time
  // Before first tp => None
  assert!(beatmap.locate_timing_point(Millis(0)).is_none());
  assert!(beatmap.locate_timing_point(Millis(410)).is_none()); // tp is AT 410, not before

  // After first tp => Some
  let tp = beatmap
    .locate_timing_point(Millis(500))
    .expect("tp after 410ms");
  assert_eq!(tp.time, Millis(410));

  // locate_hitobject: exact-time match
  let first_ho = beatmap.hit_objects.first().expect("has hit objects");
  let t = first_ho.start_time;
  assert!(beatmap.locate_hitobject(t).is_some());
  assert!(beatmap.locate_hitobject(Millis(t.0 + 1)).is_none());

  Ok(())
}
