use eframe::wgpu::naga::TypeInner::Array;
use ndarray::{array, Array3};
use image::RgbImage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Preset {
    inner: UnprocessedPreset,
    assignments: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
struct UnprocessedPreset {
    name: String,
    width: u32,
    height: u32,
    source_img: Vec<u8>,
}
enum ProgressMsg {
    Progress(f32),
    UpdatePreview {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
    UpdateAssignments(Vec<usize>),
    Done(Preset), // result directory
    Error(String),
    Cancelled,
}

impl ProgressMsg {
    fn typ(&self) -> &'static str {
        match self {
            ProgressMsg::Progress(_) => "progress",
            ProgressMsg::UpdatePreview { .. } => "update_preview",
            ProgressMsg::UpdateAssignments(_) => "update_assignments",
            ProgressMsg::Done(_) => "done",
            ProgressMsg::Error(_) => "error",
            ProgressMsg::Cancelled => "cancelled",
        }
    }
}
trait ProgressSink {
    fn send(&mut self, msg: ProgressMsg);
}
// Native-friendly adapter
impl ProgressSink for std::sync::mpsc::SyncSender<ProgressMsg> {
    fn send(&mut self, msg: ProgressMsg) {
        let _ = std::sync::mpsc::SyncSender::send(self, msg);
    }
}
#[derive(Clone, Copy)]
struct Pixel {
    src_x: u16,
    src_y: u16,
    rgb: (u8, u8, u8),
    h: i64, // current heuristic value
}
#[inline(always)]
fn heuristic(
    apos: (u16, u16),
    bpos: (u16, u16),
    a: (u8, u8, u8),
    b: (u8, u8, u8),
    color_weight: i64,
    spatial_weight: i64,
) -> i64 {
    let spatial = (apos.0 as i64 - bpos.0 as i64).pow(2) + (apos.1 as i64 - bpos.1 as i64).pow(2);
    let color = (a.0 as i64 - b.0 as i64).pow(2)
        + (a.1 as i64 - b.1 as i64).pow(2)
        + (a.2 as i64 - b.2 as i64).pow(2);
    color * color_weight + (spatial * spatial_weight).pow(2)
}
impl Pixel {
    fn new(src_x: u16, src_y: u16, rgb: (u8, u8, u8), h: i64) -> Self {
        Self {
            src_x,
            src_y,
            rgb,
            h,
        }
    }

    fn update_heuristic(&mut self, new_h: i64) {
        self.h = new_h;
    }

    #[inline(always)]
    fn calc_heuristic(
        &self,
        target_pos: (u16, u16),
        target_col: (u8, u8, u8),
        weight: i64,
        proximity_importance: i64,
    ) -> i64 {
        heuristic(
            (self.src_x, self.src_y),
            target_pos,
            self.rgb,
            target_col,
            weight,
            proximity_importance,
        )
    }
}
pub fn process_genetic(img: RgbImage){
    let raw = img.into_raw();
    let matrix: Array3<u8> = Array3::from_shape_vec((256, 256, 1), raw).unwrap();
    
}