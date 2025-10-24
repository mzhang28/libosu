use crate::beatmap::Beatmap;
use crate::hitobject::{HitObject, HitObjectKind, SpinnerInfo};
use crate::timing::{
  InheritedTimingInfo, Millis, TimingPoint, TimingPointKind,
  UninheritedTimingInfo,
};

impl Beatmap {
  /// Get the maximum combo in this map
  // TODO: verify this is accurate

  // TODO: take these allows away once this is confirmed
  #[allow(unused_variables, unused_assignments)]
  pub fn max_combo(&self) -> u32 {
    let mut res = 0;

    let mut mpb = 0.0;
    let mut sv = 0.0;
    for (i, (obj, tp)) in self.double_iter().enumerate() {
      let sl = match &obj.kind {
        // trivial case of circle or spinner
        HitObjectKind::Circle | HitObjectKind::Spinner(_) => {
          res += 1;
          continue;
        }
        HitObjectKind::Slider(v) => v,
      };

      match &tp.kind {
        TimingPointKind::Inherited(tp) => sv = tp.slider_velocity,
        TimingPointKind::Uninherited(tp) => mpb = tp.mpb,
      };
      let slider_multiplier = self.difficulty.slider_multiplier;
      let pixels_per_beat = slider_multiplier * 100.0 * sv;

      let num_beats =
        (sl.pixel_length * sl.num_repeats as f64) / pixels_per_beat;
      let mut ticks = ((num_beats - 0.1) / sl.num_repeats as f64
        * self.difficulty.slider_tick_rate)
        .ceil() as i32;
      ticks -= 1;
      ticks *= sl.num_repeats as i32;
      ticks += sl.num_repeats as i32 + 1;
      res += ticks.max(0) as u32;
    }

    res
  }

  /// Iterate over both hit objects and timing points. See [`DoubleIter`] for more info.
  pub fn double_iter(&self) -> DoubleIter<'_> {
    DoubleIter::new(self)
  }

  /// Computes the end time of the given hitobject
  pub fn get_hitobject_end_time(&self, ho: &HitObject) -> Option<f64> {
    match ho.kind {
      HitObjectKind::Circle => Some(ho.start_time.as_seconds()),
      HitObjectKind::Slider(_) => {
        let duration = self.get_slider_duration(ho)?;
        Some(ho.start_time.as_seconds() + duration)
      }
      HitObjectKind::Spinner(SpinnerInfo { end_time }) => {
        Some(end_time.as_seconds())
      }
    }
  }

  /// Returns the slider duration in seconds (including repeats!) for a given slider
  pub fn get_slider_duration(&self, ho: &HitObject) -> Option<f64> {
    let info = match &ho.kind {
      HitObjectKind::Slider(info) => info,
      _ => return None,
    };

    let slider_velocity = self.get_slider_velocity_at_time(ho.start_time);
    let slider_multiplier = self.difficulty.slider_multiplier;
    let pixels_per_beat = slider_multiplier * 100.0 * slider_velocity;
    let beats_number =
      info.pixel_length * info.num_repeats as f64 / pixels_per_beat;

    let bpm = self.get_bpm_at_time(ho.start_time)?;
    let beat_duration = 60.0 / bpm;
    let duration = beats_number * beat_duration;

    Some(duration)
  }

  /// Returns the slider velocity at the given time
  pub fn get_slider_velocity_at_time(&self, time: Millis) -> f64 {
    // TODO: replace this with binary search
    let mut current = 1.0;

    // assume this is sorted
    for tp in self.timing_points.iter() {
      if tp.time > time {
        break;
      }

      match &tp.kind {
        TimingPointKind::Uninherited(_) => {
          current = 1.0;
        }
        TimingPointKind::Inherited(InheritedTimingInfo {
          slider_velocity,
          ..
        }) => {
          current = *slider_velocity;
        }
      }
    }

    current
  }

  /// Returns the BPM at the given time
  pub fn get_bpm_at_time(&self, time: Millis) -> Option<f64> {
    // TODO: replace this with binary search
    let mut current = None;

    // assume this is sorted
    for tp in self.timing_points.iter() {
      if tp.time > time {
        break;
      }

      if let TimingPointKind::Uninherited(UninheritedTimingInfo {
        mpb, ..
      }) = tp.kind
      {
        current = Some(60_000.0 / mpb as f64);
      }
    }

    current
  }
}

/// An iterator over both hit objects and their corresponding timing points
///
/// For each hit object, the timing point that most closely affects it will be returned in the same
/// tuple.
pub struct DoubleIter<'a> {
  beatmap: &'a Beatmap,
  ho_index: usize,
  tp_index: usize,
}

impl<'a> DoubleIter<'a> {
  fn new(beatmap: &'a Beatmap) -> Self {
    DoubleIter {
      beatmap,
      ho_index: 0,
      tp_index: 0,
    }
  }
}

impl<'a> Iterator for DoubleIter<'a> {
  type Item = (&'a HitObject, &'a TimingPoint);

  fn next(&mut self) -> Option<Self::Item> {
    // try to get the next hit object
    let ho = match self.beatmap.hit_objects.get(self.ho_index) {
      Some(v) => v,
      None => return None,
    };

    let tp = loop {
      // get the currently tracked tp
      let this_tp = match self.beatmap.timing_points.get(self.tp_index) {
        Some(v) => v,
        None => return None,
      };

      if let Some(v) = self.beatmap.timing_points.get(self.tp_index + 1) {
        if v.time <= ho.start_time {
          self.tp_index += 1;
          continue;
        }
      }

      break this_tp;
    };

    self.ho_index += 1;
    Some((ho, tp))
  }
}
