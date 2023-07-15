#![feature(get_mut_unchecked)]

use std::sync::Arc;

use pipelines::ui::PipelineUI;

pub mod pipelines;
pub mod vertex;
pub mod blocks;

pub fn add_buckets(
	scene: &mut vpe::Scene,
	camera_state: &Arc<dyn vpe::Camera>,
) {
	scene.add_bucket("", |program_data| {
		Arc::new(PipelineUI::new(
			program_data,
			camera_state.clone(),
		))
	});
}