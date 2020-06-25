use super::Film;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Builder {
  pub size: (u32, u32),
}

impl From<Builder> for Film {
  fn from(fb: Builder) -> Self {
    let (w, h) = fb.size;
    Film::empty(w, h)
  }
}
