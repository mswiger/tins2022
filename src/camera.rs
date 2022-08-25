use super::app::AppState;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{Camera, RenderTarget},
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_resource::*,
        renderer::RenderQueue,
        view::RenderLayers,
        RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d},
};
use std::cmp::min;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 360;

#[derive(Default)]
pub struct ScreenImage(Handle<Image>);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenImage>()
            .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_cameras));
        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Prepare, prepare_post_processing_material);
    }
}

fn setup_cameras(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);

    commands.insert_resource(ScreenImage(image_handle.clone()));

    let scale = min(
        window.physical_width() / GAME_WIDTH,
        window.physical_height() / GAME_HEIGHT,
    );

    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            projection: bevy::render::camera::OrthographicProjection {
                scale: 1. / (scale as f32),
                ..default()
            },
            transform: Transform::from_xyz(280., 152., 999.),
            ..default()
        })
        .insert(UiCameraConfig { show_ui: false });

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        time_since_startup: 0.,
        source_image: image_handle,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        })
        .insert(post_processing_layer);

    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                priority: 1,
                ..default()
            },
            projection: bevy::render::camera::OrthographicProjection {
                scale: 1. / (scale as f32),
                ..default()
            },
            transform: Transform::from_xyz(280., 152., 999.),
            ..default()
        })
        .insert(post_processing_layer);
}

fn prepare_post_processing_material(
    materials: Res<RenderMaterials2d<PostProcessingMaterial>>,
    query: Query<&Handle<PostProcessingMaterial>>,
    time: Res<ExtractedTime>,
    render_queue: Res<RenderQueue>,
) {
    for handle in &query {
        if let Some(material) = materials.get(handle) {
            for binding in material.bindings.iter() {
                if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                    let mut buffer = encase::UniformBuffer::new(Vec::new());
                    buffer
                        .write(&PostProcessingMaterialUniformData {
                            time_since_startup: time.time_since_startup,
                        })
                        .unwrap();
                    render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
                }
            }
        }
    }
}

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "1c4b51b9-bf23-4fd5-8e80-febc028fdb4e"]
struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
    #[uniform(2)]
    time_since_startup: f32,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }
}

#[derive(Clone, ShaderType)]
struct PostProcessingMaterialUniformData {
    time_since_startup: f32,
}

struct ExtractedTime {
    time_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime {
            time_since_startup: time.seconds_since_startup() as f32,
        }
    }
}
