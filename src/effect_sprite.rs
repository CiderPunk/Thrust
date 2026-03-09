use bevy::{
  mesh::MeshTag, platform::collections::HashMap, prelude::*, render::render_resource::{AsBindGroup, ShaderType}, shader::ShaderRef
};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use crate::{asset_management::AssetLoadState, game_state::GameState};

const MAX_EFFECT_FRAMES:usize = 50;

const EFFECT_SPRITE_SHADER_PATH: &str = "shaders/animated_uv.wgsl";

const EFFECT_TYPE_SPLOSION:&'static str = "splosion";


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectSpriteType {
  Splosion,
}


pub struct EffectSpritePlugin;

impl Plugin for EffectSpritePlugin{
  fn build(&self, app: &mut App) {
    app
      
      .init_resource::<EffectDefinitionCollection>()
      .init_resource::<EffectMaterials>()
      .add_plugins(JsonAssetPlugin::<SpriteMapData>::new(&["json"]))
      .add_plugins(MaterialPlugin::<EffectSpriteMaterial>::default())
      .configure_loading_state(
        LoadingStateConfig::new(AssetLoadState::Startup)
          .load_collection::<SpriteSheetAssets>(),
      )
      .add_message::<EffectSpriteMessage>()
      .add_systems(Startup, (init_effect_definitions, init_mesh))
      .add_systems(OnEnter(AssetLoadState::Startup), load_sprite_sheets)
      .add_systems(OnEnter(AssetLoadState::Loaded), init_sprite_sheets)
      .add_systems(OnEnter(GameState::Initialize), spawn_effect_sprites);
  }
}

#[derive(Resource)]
struct EffectSpriteDefinition {
  effect_type: &'static str,
  path: &'static str,
  frame_rate: f32,
}

#[derive(Resource, Default)]
struct EffectDefinitionCollection(Vec<EffectSpriteDefinition>);


fn init_effect_definitions(
  mut effects:ResMut<EffectDefinitionCollection>,
){
  effects.0.push(
   EffectSpriteDefinition {
    effect_type: EFFECT_TYPE_SPLOSION,
    path: "effect_sprites/splosion1",
    frame_rate:30.
  });
  effects.0.push(
   EffectSpriteDefinition {
    effect_type: EFFECT_TYPE_SPLOSION,
    path: "effect_sprites/splosion2",
    frame_rate:30.
  });
  effects.0.push(
   EffectSpriteDefinition {
    effect_type: EFFECT_TYPE_SPLOSION,
    path: "effect_sprites/splosion3",
    frame_rate:30.
  });
}

fn load_sprite_sheets(
  effect_defs:Res<EffectDefinitionCollection>,
  mut dynamic_assets: ResMut<DynamicAssets>,
){ 
  let image_paths = effect_defs.0.iter().map(|f| format!("{}.png", f.path)).collect();
  let map_paths = effect_defs.0.iter().map(|f| format!("{}.json", f.path)).collect();
  dynamic_assets.register_asset(
    "spritesheet_images",
    Box::new(StandardDynamicAsset::Files{ paths: image_paths })
  );
  dynamic_assets.register_asset(
    "spritesheet_maps",
    Box::new(StandardDynamicAsset::Files{ paths: map_paths })
  );
}

#[derive(Resource)]
struct EffectQuad(Handle<Mesh>);

#[derive(Message)]
pub struct EffectSpriteMessage {
  effect_type: String,
  translation: Vec3,
  scale: f32,
  velocity: Vec3,
}

impl EffectSpriteMessage {
  pub fn new(effect_type:String, translation: Vec3, scale: f32, velocity: Vec3) -> Self {
    Self {
      effect_type,
      translation,
      scale,
      velocity,
    }
  }
}


fn spawn_effect_sprites(
  mut reader:MessageReader<EffectSpriteMessage>,
  mut commands: Commands,
  mesh: Res<EffectQuad>,
  effects: Res<EffectMaterials>,
  time: Res<Time>,
) {
  let offset: u32 = time.elapsed_secs_wrapped().to_bits();
  for effect_message in reader.read(){
    info!("spawning effect");
    commands.spawn((
      Mesh3d(mesh.0.clone()),
    //  MeshMaterial3d(effects.splosion.clone()),
      Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(10.)),
      MeshTag(offset),
    ));

    }  
    
}



fn init_mesh(
  mut meshes: ResMut<Assets<Mesh>>,
  mut commands:Commands,
){
  //let quad = meshes.add(create_quad());
   let quad =  meshes.add(Plane3d::new(Vec3::Z, Vec2::new(1.,1.)));
  //let quad = meshes.add(Sphere::new(2.).mesh().uv(32, 18));
  commands.insert_resource(EffectQuad(quad));
}

fn init_sprite_sheets(
  effect_defs:Res<EffectDefinitionCollection>,
  sprite_resources:Res<SpriteSheetAssets>,
  sprite_map_assets: Res<Assets<SpriteMapData>>,
  mut materials: ResMut<Assets<EffectSpriteMaterial>>,
  mut effect_materials:ResMut<EffectMaterials>,
){
  for (i,effect_def) in effect_defs.0.iter().enumerate(){
    let material = init_sprite_sheet(sprite_resources.maps[i].clone(), sprite_resources.images[i].clone(), effect_def.frame_rate, sprite_map_assets, materials);


  }

}


fn init_sprite_sheet(
  sprite_sheet_map:Handle<SpriteMapData>,
  sprite_sheet_image:Handle<Image>,
  frame_rate:f32,
  sprite_map_assets: &Res<Assets<SpriteMapData>>,
  mut materials: ResMut<Assets<EffectSpriteMaterial>>,
) -> Handle<EffectSpriteMaterial>{
  let Some(sprite_map) = sprite_map_assets.get(sprite_sheet_map.id()) else {
    panic!("Failed loading sprite map");
  };
  //sort the frames by filename frame number!
  let mut sortable:Vec<(u32,&FrameData)> = sprite_map.frames.iter().map(|frame|{ 
    let sort = frame.filename.chars()
      .skip_while(|c| !c.is_numeric())  
      .collect::<String>()
      .parse()
      .unwrap_or(0);
    (sort, frame) 
  })
  .collect();
  sortable.sort_unstable_by_key(|(k,_)| *k);

  let frame_count = sprite_map.frames.len();
  let canvas_size = Vec2{ x: sprite_map.meta.size.w as f32, y: sprite_map.meta.size.h as f32 };
  let mut frame_defs = [FrameDefinition { uv_rect: Vec4::ZERO, trim_rect: Vec4::ZERO }; MAX_EFFECT_FRAMES];

  for (i,(_, frame)) in sortable.iter().enumerate(){
    let tile_size = Vec2{ x: frame.source_size.w as f32, y: frame.source_size.h as f32 };
    let uv_rect = Vec4::new(
      frame.frame.x as f32 / canvas_size.x, 
      frame.frame.y as f32 / canvas_size.y,
      (frame.frame.x + frame.frame.w) as f32 / canvas_size.x,
      (frame.frame.y + frame.frame.h) as f32 / canvas_size.y,
    ); 
    let trim_rect = Vec4::new(
      ((frame.sprite_source_size.x as f32 / tile_size.x) - 0.5) * -2.,
      ((frame.sprite_source_size.y as f32 / tile_size.y) - 0.5)  * -2.,
      (((frame.sprite_source_size.x + frame.sprite_source_size.w) as f32 / tile_size.x)- 0.5)  * 2.,
      (((frame.sprite_source_size.y + frame.sprite_source_size.h) as f32 / tile_size.y)- 0.5)  * 2.,
    );
    frame_defs[i] = FrameDefinition { uv_rect, trim_rect };
  }

  
  let material = materials.add(EffectSpriteMaterial{ 
    settings: EffectSpriteSettings { 
      frame_rate: frame_rate, 
      frame_count: frame_count as u32, 
      filler: Vec2::ZERO,
    }, 
    frames: frame_defs, 
    texture_atlas: sprite_sheet_image.clone(), 
    alpha_mode: AlphaMode::Premultiplied
   });
   (material)
}

#[derive(Resource, Default)]
pub struct EffectMaterials(HashMap<String, Vec<EffectSpriteMaterial>>);


#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct EffectSpriteSettings {
  frame_rate: f32,
  frame_count: u32,
  filler: Vec2,
}

#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct FrameDefinition{
  uv_rect:Vec4,
  trim_rect:Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EffectSpriteMaterial {
  #[uniform(0)]
  settings: EffectSpriteSettings,
  #[uniform(1)]
  frames:[FrameDefinition;MAX_EFFECT_FRAMES],
  #[texture(2)]
  #[sampler(3)]
  texture_atlas: Handle<Image>,
  alpha_mode: AlphaMode,
}

impl Material for EffectSpriteMaterial {
  fn vertex_shader() -> ShaderRef {
    EFFECT_SPRITE_SHADER_PATH.into()
  }

  fn fragment_shader() -> ShaderRef {
    EFFECT_SPRITE_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteSheetAssets{
  #[asset(key="spritesheet_images", collection(typed))]
  images: Vec<Handle<Image>>,
  #[asset(key="spritesheet_maps", collection(typed))]
  maps:Vec<Handle<SpriteMapData>>,
}


// region: sprite map

#[derive(Resource)]
struct SpriteSheetMap(Handle<SpriteMapData>);

#[derive(serde::Deserialize, Asset, TypePath)]
struct SpriteMapData {
  frames: Vec<FrameData>,
  meta: MetaData,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct MetaData{
  app: String,
  version: String,
  image: String, 
  format: String,
  size: Size,
  scale: f32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct Size{
  w:u32,
  h:u32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct Point{
  x:f32,
  y:f32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct Rect{
  x:u32,
  y:u32,
  w:u32,
  h:u32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct FrameData{
  filename:String,
  rotated:bool,
  trimmed:bool,
  frame:Rect,
  #[serde(rename = "spriteSourceSize")]
  sprite_source_size:Rect,
  #[serde(rename = "sourceSize")]
  source_size:Size,
  pivot:Point,
}

// endregion
