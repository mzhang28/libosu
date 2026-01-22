use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use libosu::{
  beatmap::Beatmap,
  data::Mode,
  events::{BackgroundEvent, BreakEvent, Event},
  hitsounds::SampleSet,
  math::Point,
  timing::Millis,
};

fn load_beatmap(path: impl AsRef<Path>) -> Beatmap {
  let mut file = File::open(path.as_ref()).expect("couldn't open file");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("couldn't read file");
  Beatmap::from_str(&contents).expect("couldn't parse")
}

#[test]
fn parse_taeyang_remote_control() {
  let beatmap = load_beatmap("tests/files/774965.osu");

  assert_eq!(beatmap.audio_filename, "control.mp3");
  assert_eq!(beatmap.audio_leadin, Millis(1000));
  assert_eq!(beatmap.preview_time, Millis(85495));
  assert_eq!(beatmap.countdown, false);
  assert_eq!(beatmap.sample_set, SampleSet::Normal);
  assert_eq!(beatmap.stack_leniency, 0.8);
  assert_eq!(beatmap.mode, Mode::Osu);
  assert_eq!(beatmap.letterbox_in_breaks, false);
  assert_eq!(beatmap.widescreen_storyboard, false);

  assert_eq!(beatmap.title, "Remote Control");
  assert_eq!(beatmap.title_unicode, "リモコン");
  assert_eq!(beatmap.artist, "kradness&Reol");
  assert_eq!(beatmap.artist_unicode, "kradness＆れをる");
  assert_eq!(beatmap.creator, "Taeyang");
  assert_eq!(beatmap.difficulty_name, "Max Control!");
  assert_eq!(beatmap.source, "");
  assert_eq!(
    beatmap.tags,
    &[
      "Jesus-P",
      "じーざすP",
      "Giga",
      "Official",
      "Rimokon",
      "Wonderful*Opportunity",
      "Kagamine",
      "Rin",
      "Len",
      "Glider"
    ]
  );
  assert_eq!(beatmap.beatmap_id, 774965);
  assert_eq!(beatmap.beatmap_set_id, 351630);

  assert_eq!(
    beatmap.events,
    &[
      Event::Background(BackgroundEvent {
        filename: String::from("reol.jpg"),
        offset: Point::new(0, 0)
      }),
      Event::Break(BreakEvent {
        start_time: Millis(184604),
        end_time: Millis(189653),
      })
    ]
  );
}

macro_rules! test_serde {
    ($($name:ident: $id:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut file = File::open(format!("tests/files/{}.osu", $id)).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");

                let beatmap = Beatmap::from_str(&contents).expect("couldn't parse");
                let reexported = beatmap.to_string();

                let beatmap2 = match Beatmap::from_str(&reexported) {
                    Ok(v) => v,
                    Err(err) => {
                        for (i, line) in reexported.lines().enumerate() {
                            let line_no = i as i32 + 1;
                            if (line_no - err.line as i32).abs() < 3 {
                                eprintln!("{}:\t{}", line_no, line);
                            }
                        }
                        panic!("error: {}", err);
                    }
                };

                assert_eq!(beatmap, beatmap2);
            }
        )*
    };
}

test_serde! {
    // test_parser_75: 75,
    test_parser_129891: 129891,
    test_parser_774965: 774965,
    test_parser_804683: 804683,
    test_parser_1595588: 1595588,

    // https://github.com/mzhang28/libosu/issues/12
    test_parser_1360: 1360,
    test_parser_3516: 3516,

    // https://github.com/mzhang28/libosu/issues/13
    test_parser_169355: 169355,

    // https://github.com/mzhang28/libosu/issues/24
    test_parser_1513248: 1513248,
}
