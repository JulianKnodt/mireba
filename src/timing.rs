use std::{collections::HashMap, sync::Mutex, time::Duration};

#[derive(Default, Clone, Debug)]
pub struct Timings {
  pub(crate) items: HashMap<&'static str, (u32, Duration)>,
}

thread_local! {
  pub static TIMER: Mutex<Timings> = Mutex::new(Default::default());
}

#[allow(unused)]
macro_rules! time {
  ($name: ident, $to_time: block) => {{
    use std::time::Instant;
    let now = Instant::now();
    let result = $to_time;
    let elapsed = now.elapsed();
    {
      crate::timing::TIMER.with(|v| {
        let mut inner = v.lock().unwrap();
        inner
          .items
          .entry(stringify!($name))
          .and_modify(|e| {
            if e.0 > 300 {
              return;
            }
            let next = (e.1 * e.0) + elapsed;
            e.0 += 1;
            e.1 = next / e.0;
          })
          .or_insert((1, elapsed));
      });
    };
    result
  }};
  ($name: ident, $to_time: expr) => {
    time!($name, { $to_time });
  };
}

#[test]
fn test_timing() {
  time!(Add, {
    let _x = 3 + 3;
  });
  TIMER.with(|v| {
    let inner = v.lock().unwrap();
    assert_eq!(inner.items[stringify!(Add)].0, 1);
    assert_ne!(inner.items[stringify!(Add)].1.as_nanos(), 0);
  });
}

/// Prints all timed values from this session
pub fn print_timed() {
  TIMER.with(|v| {
    let inner = v.lock().unwrap();
    inner.items.iter().for_each(|(k, v)| {
      println!("{}: Average {:?}", k, v.1);
    });
  });
}
