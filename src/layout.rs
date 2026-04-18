use std::fs;

pub fn build_svg(w: u32, h: u32) -> String {
    let css = fs::read_to_string("style.css").expect("style.css not found");

    let lx: u32 = 160;
    let ly: u32 = h / 2;
    let cx: u32 = 400;
    let cy: u32 = h / 2;
    let rx: u32 = 640;
    let ry: u32 = h / 2;

    let c_rect_ox = cx - 30;
    let c_rect_oy = cy.saturating_sub(30);
    let c_rect_ix = cx - 15;
    let c_rect_iy = cy.saturating_sub(15);

    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     width="{w}" height="{h}"
     viewBox="0 0 {w} {h}"
     preserveAspectRatio="xMidYMid meet">

  <style>{css}</style>

  <!-- background -->
  <rect width="{w}" height="{h}" fill="#0d1117" />

  <!-- subtle grid -->
  <line class="grid-line" x1="0"    y1="{ly}" x2="{w}"  y2="{ly}" style="animation-delay:0s"  />
  <line class="grid-line" x1="{cx}" y1="0"    x2="{cx}" y2="{h}"  style="animation-delay:2s"  />

  <!-- left cluster -->
  <circle class="l-ring-a" cx="{lx}" cy="{ly}" r="55" />
  <circle class="l-ring-b" cx="{lx}" cy="{ly}" r="36" />
  <circle class="l-ring-c" cx="{lx}" cy="{ly}" r="18" />
  <circle class="l-dot"    cx="{lx}" cy="{ly}" r="3"  />

  <!-- center cluster -->
  <line class="c-cross-h" x1="{c_cross_x1}" y1="{cy}" x2="{c_cross_x2}" y2="{cy}" />
  <line class="c-cross-v" x1="{cx}" y1="{c_cross_y1}" x2="{cx}" y2="{c_cross_y2}" />
  <rect class="c-rect-outer" x="{c_rect_ox}" y="{c_rect_oy}" width="60" height="60" />
  <rect class="c-rect-inner" x="{c_rect_ix}" y="{c_rect_iy}" width="30" height="30" />
  <circle class="c-dot" cx="{cx}" cy="{cy}" r="3" />

  <!-- right cluster -->
  <circle class="r-ring-a" cx="{rx}" cy="{ry}" r="50" />
  <circle class="r-ring-b" cx="{rx}" cy="{ry}" r="28" />
  <circle class="r-dot"    cx="{rx}" cy="{ry}" r="3"  />

  <!-- connectors -->
  <line class="connector" x1="{l_conn_x}" y1="{ly}" x2="{c_conn_lx}" y2="{cy}" opacity="0.15" />
  <line class="connector" x1="{c_conn_rx}" y1="{cy}" x2="{r_conn_x}" y2="{ry}" opacity="0.15" />

</svg>"##,
        w = w, h = h,
        lx = lx, ly = ly,
        cx = cx, cy = cy,
        rx = rx, ry = ry,
        css = css,
        c_cross_x1 = cx - 50, c_cross_x2 = cx + 50,
        c_cross_y1 = cy.saturating_sub(50), c_cross_y2 = cy + 50,
        c_rect_ox = c_rect_ox, c_rect_oy = c_rect_oy,
        c_rect_ix = c_rect_ix, c_rect_iy = c_rect_iy,
        l_conn_x  = lx + 55,
        c_conn_lx = cx - 30,
        c_conn_rx = cx + 30,
        r_conn_x  = rx - 50,
    )
}
