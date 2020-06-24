use super::ImageBlock;

/// Creates blocks from top-left per column.
pub fn naive((w, h): (u32, u32), (rows, cols): (u32, u32)) -> Vec<ImageBlock> {
  let num_per_row = w / rows;
  assert!(num_per_row > 0);

  let num_per_col = h / cols;
  assert!(num_per_col > 0);
  (0..cols)
    .flat_map(move |xth| {
      let x_start = xth * num_per_col;
      let x_end = if xth == cols - 1 {
        w
      } else {
        (xth + 1) * num_per_col
      };
      (0..rows).map(move |yth| {
        let y_start = yth * num_per_row;
        let y_end = if yth == rows - 1 {
          h
        } else {
          (yth + 1) * num_per_row
        };
        ImageBlock::new((x_end - x_start, y_end - y_start), (x_start, y_start))
      })
    })
    .collect()
}
