use bevy::{
  prelude::*, render::render_resource::{AsBindGroup, BufferUsages, BufferVec, ShaderType}, shader::ShaderRef
};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use crate::asset_management::AssetLoadState;


const EFFECT_SPRITE_SHADER_PATH: &str = "shaders/animated_uv.wgsl";

pub struct EffectSpritePlugin;

impl Plugin for EffectSpritePlugin{
  fn build(&self, app: &mut App) {
    app
      .init_resource::<EffectMaterials>()
      .add_plugins(JsonAssetPlugin::<SpriteMapData>::new(&["map.json"]))
      .add_plugins(MaterialPlugin::<EffectSpriteMaterial>::default())
      .configure_loading_state(
        LoadingStateConfig::new(AssetLoadState::Startup)
          .load_collection::<SpriteSheetAssets>(),
      )
      .add_systems(OnEnter(AssetLoadState::Loaded), init_sprite_sheets);
  }
}

fn init_sprite_sheets(
  sprite_sheets:Res<SpriteSheetAssets>,
  sprite_map_assets: Res<Assets<SpriteMapData>>,
){
  let Some(sprite_map) = sprite_map_assets.get(sprite_sheets.splosion_map.id()) else {
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

let mut buffer = BufferVec::<[f32;2]>::new(BufferUsages::empty());

  for (_, frame) in sortable{

  }
  
}


#[derive(Resource, Default)]
pub struct EffectMaterials{
  pub splosion:Handle<EffectSpriteMaterial>,
}



#[derive(Default, Clone, Copy, AsBindGroup, Debug, ShaderType)]
pub struct EffectSpriteSettings {
  frame_rate: f32,
  frames_wide: f32,
  frames_deep: f32,
  frame_count: f32,
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EffectSpriteMaterial {
  #[uniform(0)]
  settings: EffectSpriteSettings,
  #[texture(1)]
  #[sampler(2)]
  texture_atlas: Option<Handle<Image>>,
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
  #[asset(path="spritesheets/splosion.png")]
  pub splosion_image: Handle<Image>,
  #[asset(path="spritesheets/splosion.map.json")]
  pub splosion_map:Handle<SpriteMapData>,
}


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

