use std::sync::Arc;

use ash::vk;

use crate::{vertex::ui::VertexUI, blocks::ui::BlockUI};

pub struct PipelineUI {
	pipeline_info: Arc<vpe::PipelineInfo>,
	pipeline_block_structure: Arc<vpe::ObjectBlockStructure>,
	object_block_structure: Arc<vpe::ObjectBlockStructure>,
	camera: Arc<dyn vpe::Camera>,
}

impl PipelineUI {
	pub fn new(
		program_data: &vpe::ProgramData,
		camera: Arc<dyn vpe::Camera>,
	) -> Self { unsafe {
		let pipeline_block_structure = Arc::new(vpe::ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<vpe::BlockCamera2d>::new(
					&program_data.device,
					0, 0,
				))
			],
		});
		let object_block_structure = Arc::new(vpe::ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<BlockUI>::new(
					&program_data.device,
					0, 1,
				))
			],
		});
		let pipeline_info = Arc::new(vpe::PipelineInfo::new::<VertexUI>(
			program_data,
			"ub_ui",
			true,
			vpe::ViewportDepthRange::UI,
			vk::PolygonMode::FILL,
			&pipeline_block_structure,
			&object_block_structure,
		));
		Self {
			pipeline_info,
			pipeline_block_structure,
			object_block_structure,
			camera,
		}
	}}
}

impl vpe::EnginePipeline for PipelineUI {
	fn get_pipeline_info(
		&self,
	) -> Arc<vpe::PipelineInfo> {
		self.pipeline_info.clone()
	}

	fn get_pipeline_block_structure(
		&self,
	) -> Arc<vpe::ObjectBlockStructure> {
		self.pipeline_block_structure.clone()
	}

	fn get_object_block_structure(
		&self,
	) -> Arc<vpe::ObjectBlockStructure> {
		self.object_block_structure.clone()
	}

	fn recreate_pipeline(
		&mut self,
		program_data: &vpe::ProgramData,
	) {
		let pipeline_info = vpb::gmuc!(self.pipeline_info);
		pipeline_info.recreate_pipeline::<VertexUI>(
			program_data,
			&self.pipeline_block_structure,
			&self.object_block_structure,
		);
	}

	fn update_block_states(
		&mut self,
		program_data: &vpe::ProgramData,
		input_state: &vpe::InputState,
		render_state: &vpe::RenderState,
	) {
		self.camera.update(
			&program_data.device,
			Some(render_state.frame),
			&self.pipeline_info.block_states[0],
		)
	}
}