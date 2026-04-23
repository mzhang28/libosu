use std::str::FromStr;

use libosu_lib::beatmap::{Beatmap, BeatmapParseError, Difficulty};
use libosu_lib::color::Color;
use libosu_lib::data::{Mode, Mods};
use libosu_lib::events::Event;
use libosu_lib::hitobject::{HitObject, HitObjectKind, SliderInfo, SliderSplineKind, SpinnerInfo};
use libosu_lib::hitsounds::{Additions, SampleInfo, SampleSet};
use libosu_lib::timing::TimingPoint;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Color", from_py_object)]
#[derive(Clone)]
struct PyColor {
    inner: Color,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyColor {
    #[getter]
    fn red(&self) -> u8 {
        self.inner.red
    }
    #[getter]
    fn green(&self) -> u8 {
        self.inner.green
    }
    #[getter]
    fn blue(&self) -> u8 {
        self.inner.blue
    }

    fn __repr__(&self) -> String {
        format!(
            "Color(r={}, g={}, b={})",
            self.inner.red, self.inner.green, self.inner.blue
        )
    }
}

// ---------------------------------------------------------------------------
// Mods
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Mods", from_py_object)]
#[derive(Clone)]
struct PyMods {
    inner: Mods,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyMods {
    #[staticmethod]
    fn from_bits(bits: u32) -> Self {
        PyMods {
            inner: Mods::from_bits_truncate(bits),
        }
    }

    fn bits(&self) -> u32 {
        self.inner.bits()
    }

    #[getter]
    fn no_fail(&self) -> bool { self.inner.contains(Mods::NoFail) }
    #[getter]
    fn easy(&self) -> bool { self.inner.contains(Mods::Easy) }
    #[getter]
    fn hidden(&self) -> bool { self.inner.contains(Mods::Hidden) }
    #[getter]
    fn hard_rock(&self) -> bool { self.inner.contains(Mods::HardRock) }
    #[getter]
    fn sudden_death(&self) -> bool { self.inner.contains(Mods::SuddenDeath) }
    #[getter]
    fn double_time(&self) -> bool { self.inner.contains(Mods::DoubleTime) }
    #[getter]
    fn relax(&self) -> bool { self.inner.contains(Mods::Relax) }
    #[getter]
    fn half_time(&self) -> bool { self.inner.contains(Mods::HalfTime) }
    #[getter]
    fn nightcore(&self) -> bool { self.inner.contains(Mods::Nightcore) }
    #[getter]
    fn flashlight(&self) -> bool { self.inner.contains(Mods::Flashlight) }
    #[getter]
    fn autoplay(&self) -> bool { self.inner.contains(Mods::Autoplay) }
    #[getter]
    fn spun_out(&self) -> bool { self.inner.contains(Mods::SpunOut) }
    #[getter]
    fn perfect(&self) -> bool { self.inner.contains(Mods::Perfect) }

    fn __repr__(&self) -> String {
        format!("Mods(bits={:#010x})", self.inner.bits())
    }
}

// ---------------------------------------------------------------------------
// SampleSet / Additions / SampleInfo
// ---------------------------------------------------------------------------

fn sample_set_str(s: SampleSet) -> &'static str {
    match s {
        SampleSet::Default => "default",
        SampleSet::Normal => "normal",
        SampleSet::Soft => "soft",
        SampleSet::Drum => "drum",
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Additions", from_py_object)]
#[derive(Clone)]
struct PyAdditions {
    inner: Additions,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAdditions {
    #[getter]
    fn whistle(&self) -> bool { self.inner.contains(Additions::WHISTLE) }
    #[getter]
    fn finish(&self) -> bool { self.inner.contains(Additions::FINISH) }
    #[getter]
    fn clap(&self) -> bool { self.inner.contains(Additions::CLAP) }

    fn bits(&self) -> u32 {
        self.inner.bits()
    }

    fn __repr__(&self) -> String {
        let mut flags = vec![];
        if self.whistle() { flags.push("WHISTLE"); }
        if self.finish() { flags.push("FINISH"); }
        if self.clap() { flags.push("CLAP"); }
        if flags.is_empty() {
            "Additions(none)".to_string()
        } else {
            format!("Additions({})", flags.join("|"))
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "SampleInfo", from_py_object)]
#[derive(Clone)]
struct PySampleInfo {
    inner: SampleInfo,
}

#[gen_stub_pymethods]
#[pymethods]
impl PySampleInfo {
    #[getter]
    fn sample_set(&self) -> &str {
        sample_set_str(self.inner.sample_set)
    }
    #[getter]
    fn addition_set(&self) -> &str {
        sample_set_str(self.inner.addition_set)
    }
    #[getter]
    fn custom_index(&self) -> i32 {
        self.inner.custom_index
    }
    #[getter]
    fn sample_volume(&self) -> i32 {
        self.inner.sample_volume
    }
    #[getter]
    fn filename(&self) -> &str {
        &self.inner.filename
    }

    fn __repr__(&self) -> String {
        format!(
            "SampleInfo(sample_set={}, addition_set={}, volume={})",
            self.sample_set(),
            self.addition_set(),
            self.inner.sample_volume
        )
    }
}

// ---------------------------------------------------------------------------
// SliderInfo / SpinnerInfo
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "SliderInfo", from_py_object)]
#[derive(Clone)]
struct PySliderInfo {
    inner: SliderInfo,
}

#[gen_stub_pymethods]
#[pymethods]
impl PySliderInfo {
    #[getter]
    fn kind(&self) -> &str {
        match self.inner.kind {
            SliderSplineKind::Linear => "linear",
            SliderSplineKind::Bezier => "bezier",
            SliderSplineKind::Catmull => "catmull",
            SliderSplineKind::Perfect => "perfect",
        }
    }

    #[getter]
    fn num_repeats(&self) -> u32 {
        self.inner.num_repeats
    }

    #[getter]
    fn pixel_length(&self) -> f64 {
        self.inner.pixel_length
    }

    #[getter]
    fn control_points(&self) -> Vec<(i32, i32)> {
        self.inner.control_points.iter().map(|p| (p.x, p.y)).collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "SliderInfo(kind={}, repeats={}, length={:.1}, control_points={})",
            self.kind(),
            self.inner.num_repeats,
            self.inner.pixel_length,
            self.inner.control_points.len()
        )
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "SpinnerInfo", from_py_object)]
#[derive(Clone)]
struct PySpinnerInfo {
    inner: SpinnerInfo,
}

#[gen_stub_pymethods]
#[pymethods]
impl PySpinnerInfo {
    #[getter]
    fn end_time(&self) -> i32 {
        self.inner.end_time.0
    }

    fn __repr__(&self) -> String {
        format!("SpinnerInfo(end_time={}ms)", self.inner.end_time.0)
    }
}

// ---------------------------------------------------------------------------
// Event
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Event", from_py_object)]
#[derive(Clone)]
struct PyEvent {
    inner: Event,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEvent {
    #[getter]
    fn kind(&self) -> &str {
        match &self.inner {
            Event::Background(_) => "background",
            Event::Video(_) => "video",
            Event::Break(_) => "break",
            Event::Storyboard(_) => "storyboard",
            _ => "unknown",
        }
    }

    fn background_filename(&self) -> Option<String> {
        if let Event::Background(b) = &self.inner {
            Some(b.filename.clone())
        } else {
            None
        }
    }

    fn video_filename(&self) -> Option<String> {
        if let Event::Video(v) = &self.inner {
            Some(v.filename.clone())
        } else {
            None
        }
    }

    fn video_start_time(&self) -> Option<i32> {
        if let Event::Video(v) = &self.inner {
            Some(v.start_time.0)
        } else {
            None
        }
    }

    fn break_start_time(&self) -> Option<i32> {
        if let Event::Break(b) = &self.inner {
            Some(b.start_time.0)
        } else {
            None
        }
    }

    fn break_end_time(&self) -> Option<i32> {
        if let Event::Break(b) = &self.inner {
            Some(b.end_time.0)
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            Event::Background(b) => format!("Event::Background({})", b.filename),
            Event::Video(v) => format!("Event::Video({}, t={})", v.filename, v.start_time.0),
            Event::Break(b) => format!("Event::Break({}ms-{}ms)", b.start_time.0, b.end_time.0),
            Event::Storyboard(_) => "Event::Storyboard(...)".to_string(),
            _ => "Event::Unknown".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Difficulty
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Difficulty", from_py_object)]
#[derive(Clone)]
struct PyDifficulty {
    inner: Difficulty,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyDifficulty {
    #[getter]
    fn hp_drain_rate(&self) -> f32 { self.inner.hp_drain_rate }
    #[getter]
    fn circle_size(&self) -> f32 { self.inner.circle_size }
    #[getter]
    fn overall_difficulty(&self) -> f32 { self.inner.overall_difficulty }
    #[getter]
    fn approach_rate(&self) -> f32 { self.inner.approach_rate }
    #[getter]
    fn slider_multiplier(&self) -> f64 { self.inner.slider_multiplier }
    #[getter]
    fn slider_tick_rate(&self) -> f64 { self.inner.slider_tick_rate }

    fn circle_size_osupx(&self) -> f32 { self.inner.circle_size_osupx() }
    fn approach_preempt_ms(&self) -> i32 { self.inner.approach_preempt().0 }
    fn approach_fade_time_ms(&self) -> i32 { self.inner.approach_fade_time().0 }

    fn __repr__(&self) -> String {
        format!(
            "Difficulty(hp={}, cs={}, od={}, ar={})",
            self.inner.hp_drain_rate,
            self.inner.circle_size,
            self.inner.overall_difficulty,
            self.inner.approach_rate
        )
    }
}

// ---------------------------------------------------------------------------
// HitObject
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "HitObject", from_py_object)]
#[derive(Clone)]
struct PyHitObject {
    inner: HitObject,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyHitObject {
    #[getter]
    fn x(&self) -> i32 { self.inner.pos.x }
    #[getter]
    fn y(&self) -> i32 { self.inner.pos.y }
    #[getter]
    fn start_time(&self) -> i32 { self.inner.start_time.0 }
    #[getter]
    fn new_combo(&self) -> bool { self.inner.new_combo }
    #[getter]
    fn skip_color(&self) -> i32 { self.inner.skip_color }

    #[getter]
    fn kind(&self) -> &str {
        match &self.inner.kind {
            HitObjectKind::Circle => "circle",
            HitObjectKind::Slider(_) => "slider",
            HitObjectKind::Spinner(_) => "spinner",
        }
    }

    #[getter]
    fn additions(&self) -> PyAdditions {
        PyAdditions { inner: self.inner.additions.clone() }
    }

    #[getter]
    fn sample_info(&self) -> PySampleInfo {
        PySampleInfo { inner: self.inner.sample_info.clone() }
    }

    fn slider_info(&self) -> Option<PySliderInfo> {
        if let HitObjectKind::Slider(s) = &self.inner.kind {
            Some(PySliderInfo { inner: s.clone() })
        } else {
            None
        }
    }

    fn spinner_info(&self) -> Option<PySpinnerInfo> {
        if let HitObjectKind::Spinner(s) = &self.inner.kind {
            Some(PySpinnerInfo { inner: s.clone() })
        } else {
            None
        }
    }

    fn is_circle(&self) -> bool { self.inner.kind.is_circle() }
    fn is_slider(&self) -> bool { self.inner.kind.is_slider() }
    fn is_spinner(&self) -> bool { self.inner.kind.is_spinner() }

    fn __repr__(&self) -> String {
        format!(
            "HitObject(kind={}, x={}, y={}, t={}ms)",
            self.kind(),
            self.x(),
            self.y(),
            self.start_time()
        )
    }
}

// ---------------------------------------------------------------------------
// TimingPoint
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "TimingPoint", from_py_object)]
#[derive(Clone)]
struct PyTimingPoint {
    inner: TimingPoint,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTimingPoint {
    #[getter]
    fn time(&self) -> i32 { self.inner.time.0 }

    #[getter]
    fn bpm(&self) -> Option<f64> {
        use libosu_lib::timing::TimingPointKind;
        match &self.inner.kind {
            TimingPointKind::Uninherited(info) => Some(60_000.0 / info.mpb),
            TimingPointKind::Inherited(_) => None,
        }
    }

    #[getter]
    fn is_uninherited(&self) -> bool {
        use libosu_lib::timing::TimingPointKind;
        matches!(self.inner.kind, TimingPointKind::Uninherited(_))
    }

    fn __repr__(&self) -> String {
        if let Some(bpm) = self.bpm() {
            format!("TimingPoint(t={}ms, bpm={:.2})", self.time(), bpm)
        } else {
            format!("TimingPoint(t={}ms, inherited)", self.time())
        }
    }
}

// ---------------------------------------------------------------------------
// Beatmap
// ---------------------------------------------------------------------------

#[gen_stub_pyclass]
#[pyclass(name = "Beatmap")]
struct PyBeatmap {
    inner: Beatmap,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyBeatmap {
    #[getter]
    fn version(&self) -> u32 { self.inner.version }
    #[getter]
    fn title(&self) -> &str { &self.inner.title }
    #[getter]
    fn title_unicode(&self) -> &str { &self.inner.title_unicode }
    #[getter]
    fn artist(&self) -> &str { &self.inner.artist }
    #[getter]
    fn artist_unicode(&self) -> &str { &self.inner.artist_unicode }
    #[getter]
    fn creator(&self) -> &str { &self.inner.creator }
    #[getter]
    fn difficulty_name(&self) -> &str { &self.inner.difficulty_name }
    #[getter]
    fn audio_filename(&self) -> &str { &self.inner.audio_filename }
    #[getter]
    fn beatmap_id(&self) -> i32 { self.inner.beatmap_id }
    #[getter]
    fn beatmap_set_id(&self) -> i32 { self.inner.beatmap_set_id }
    #[getter]
    fn source(&self) -> &str { &self.inner.source }
    #[getter]
    fn tags(&self) -> Vec<String> { self.inner.tags.clone() }
    #[getter]
    fn stack_leniency(&self) -> f64 { self.inner.stack_leniency }
    #[getter]
    fn countdown(&self) -> bool { self.inner.countdown }
    #[getter]
    fn letterbox_in_breaks(&self) -> bool { self.inner.letterbox_in_breaks }

    #[getter]
    fn mode(&self) -> &str {
        match self.inner.mode {
            Mode::Osu => "osu",
            Mode::Taiko => "taiko",
            Mode::Catch => "catch",
            Mode::Mania => "mania",
        }
    }

    #[getter]
    fn difficulty(&self) -> PyDifficulty {
        PyDifficulty { inner: self.inner.difficulty.clone() }
    }

    #[getter]
    fn hit_objects(&self) -> Vec<PyHitObject> {
        self.inner.hit_objects.iter().map(|h| PyHitObject { inner: h.clone() }).collect()
    }

    #[getter]
    fn timing_points(&self) -> Vec<PyTimingPoint> {
        self.inner.timing_points.iter().map(|t| PyTimingPoint { inner: t.clone() }).collect()
    }

    #[getter]
    fn colors(&self) -> Vec<PyColor> {
        self.inner.colors.iter().map(|c| PyColor { inner: *c }).collect()
    }

    #[getter]
    fn events(&self) -> Vec<PyEvent> {
        self.inner.events.iter().map(|e| PyEvent { inner: e.clone() }).collect()
    }

    fn hit_object_count(&self) -> usize { self.inner.hit_objects.len() }
    fn circle_count(&self) -> usize { self.inner.hit_objects.iter().filter(|h| h.kind.is_circle()).count() }
    fn slider_count(&self) -> usize { self.inner.hit_objects.iter().filter(|h| h.kind.is_slider()).count() }
    fn spinner_count(&self) -> usize { self.inner.hit_objects.iter().filter(|h| h.kind.is_spinner()).count() }

    fn __repr__(&self) -> String {
        format!(
            "Beatmap({} - {} [{}] by {})",
            self.inner.artist, self.inner.title, self.inner.difficulty_name, self.inner.creator
        )
    }
}

// ---------------------------------------------------------------------------
// Parse functions
// ---------------------------------------------------------------------------

fn beatmap_parse_error(e: BeatmapParseError) -> PyErr {
    PyValueError::new_err(format!("beatmap parse error at line {}: {}", e.line, e.inner))
}

#[gen_stub_pyfunction]
#[pyfunction]
fn parse_beatmap(content: &str) -> PyResult<PyBeatmap> {
    Beatmap::from_str(content)
        .map(|inner| PyBeatmap { inner })
        .map_err(beatmap_parse_error)
}

#[gen_stub_pyfunction]
#[pyfunction]
fn parse_beatmap_file(path: &str) -> PyResult<PyBeatmap> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| PyValueError::new_err(format!("failed to read {path}: {e}")))?;
    parse_beatmap(&content)
}

// ---------------------------------------------------------------------------
// Module
// ---------------------------------------------------------------------------

#[pymodule]
fn libosu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBeatmap>()?;
    m.add_class::<PyDifficulty>()?;
    m.add_class::<PyHitObject>()?;
    m.add_class::<PyTimingPoint>()?;
    m.add_class::<PyColor>()?;
    m.add_class::<PyMods>()?;
    m.add_class::<PyAdditions>()?;
    m.add_class::<PySampleInfo>()?;
    m.add_class::<PySliderInfo>()?;
    m.add_class::<PySpinnerInfo>()?;
    m.add_class::<PyEvent>()?;
    m.add_function(wrap_pyfunction!(parse_beatmap, m)?)?;
    m.add_function(wrap_pyfunction!(parse_beatmap_file, m)?)?;
    Ok(())
}

pub fn stub_info() -> pyo3_stub_gen::Result<pyo3_stub_gen::StubInfo> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    pyo3_stub_gen::StubInfo::from_project_root(
        "libosu".to_string(),
        workspace_root,
        false,
        pyo3_stub_gen::StubGenConfig::default(),
    )
}
