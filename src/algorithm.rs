use eframe::wgpu::naga::TypeInner::Array;
use kiddo::KdTree;
use ndarray::{array, Array3};
use image::RgbImage;

fn process_optimal(img: RgbImage){
    let raw = img.into_raw();
    let matrix: Array3<u8> = Array3::from_shape_vec((256, 256, 1), raw).unwrap();
    let rows = matrix.len()/3;
    matrix.into_shape_with_order((3,rows)).unwrap();


}
fn process_genetic(img: RgbImage){

}