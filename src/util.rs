use crate::material::*;
use libsm64::{Point3, Terrain, *};
use three_d::*;
use three_d_asset::geometry::TriMesh;

const SPEED: f32 = 0.1;

// the bracketing here is so rustfmt makes it look pretty
#[macro_export]
macro_rules! input_match {
    ( [ $key:expr, $gameinput:ident, $val:literal ], $( [ $matchkey:pat, $name:ident ] ),* $(,)* ) => {
        match $key {
            $(
                $matchkey => $gameinput.update(|i| i.$name = $val),
            )*
            _ => (),
        }
    }
}

macro_rules! stick_move {
    ( $self:ident, $up:ident, $down:ident, $left:ident, $right:ident, $x:ident, $y:ident, $(,)* ) => {
        if $self.$up {
            $y -= 1.;
        }

        if $self.$down {
            $y += 1.;
        }

        if $self.$left {
            $x -= 1.;
        }

        if $self.$right {
            $x += 1.;
        }
    };
}

#[derive(Clone, Debug)]
pub struct GameInput {
    pub switch_movement: bool,
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub stick_up: bool,
    pub stick_down: bool,
    pub stick_left: bool,
    pub stick_right: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub button_z: bool,
    pub cam_x: f32,
    pub cam_z: f32,
}

impl GameInput {
    pub fn as_marioinput(&self) -> MarioInput {
        let mut stick_x = 0.;
        let mut stick_y = 0.;

        if self.switch_movement {
            stick_move!(
                self,
                arrow_up,
                arrow_down,
                arrow_left,
                arrow_right,
                stick_x,
                stick_y,
            );
        } else {
            stick_move!(
                self,
                stick_up,
                stick_down,
                stick_left,
                stick_right,
                stick_x,
                stick_y,
            );
        }

        MarioInput {
            cam_look_x: self.cam_x,
            cam_look_z: self.cam_z,
            stick_x,
            stick_y,
            button_a: self.button_a,
            button_b: self.button_b,
            button_z: self.button_z,
        }
    }

    pub fn rotate_camera(&self, camera: &mut Camera, target: Vector3<f32>) {
        let mut rotate_x = 0.;
        let mut rotate_y = 0.;

        if self.switch_movement {
            stick_move!(
                self,
                stick_up,
                stick_down,
                stick_left,
                stick_right,
                rotate_x,
                rotate_y,
            );
        } else {
            stick_move!(
                self,
                arrow_up,
                arrow_down,
                arrow_left,
                arrow_right,
                rotate_x,
                rotate_y,
            );
        }

        camera.rotate_around_with_fixed_up(target, rotate_x * SPEED, rotate_y * SPEED);
    }
}

impl Default for GameInput {
    fn default() -> Self {
        Self {
            switch_movement: false,
            arrow_up: false,
            arrow_down: false,
            arrow_left: false,
            arrow_right: false,
            stick_up: false,
            stick_down: false,
            stick_left: false,
            stick_right: false,
            button_a: false,
            button_b: false,
            button_z: false,
            cam_x: 0.,
            cam_z: 0.,
        }
    }
}

pub fn load_geometry(trimesh: &TriMesh) -> Vec<LevelTriangle> {
    let vertices = trimesh.positions.to_f32();
    let indices = trimesh.indices.to_u32().expect("we should have indices!");
    let len = indices.len() / 3;
    let mut triangles = Vec::with_capacity(len);

    for i in 0..len {
        let idx = i * 3;
        let p1 = vertices[indices[idx] as usize];
        let p2 = vertices[indices[idx + 1] as usize];
        let p3 = vertices[indices[idx + 2] as usize];

        triangles.push(LevelTriangle {
            kind: Surface::Default,
            force: 0,
            terrain: Terrain::Grass,
            vertices: (
                Point3 {
                    x: (p1.x * 100.) as i32,
                    y: (p1.y * 100.) as i32,
                    z: (p1.z * 100.) as i32,
                },
                Point3 {
                    x: (p2.x * 100.) as i32,
                    y: (p2.y * 100.) as i32,
                    z: (p2.z * 100.) as i32,
                },
                Point3 {
                    x: (p3.x * 100.) as i32,
                    y: (p3.y * 100.) as i32,
                    z: (p3.z * 100.) as i32,
                },
            ),
        })
    }

    // temp
    /*vec![
        LevelTriangle {
            kind: Surface::Default,
            force: 0,
            terrain: Terrain::Grass,
            vertices: (
                Point3 {
                    x: -1000,
                    y: 0,
                    z: 1000,
                },
                Point3 {
                    x: 1000,
                    y: 0,
                    z: 1000,
                },
                Point3 {
                    x: 1000,
                    y: 0,
                    z: -1000,
                },
            ),
        },
        LevelTriangle {
            kind: Surface::Default,
            force: 0,
            terrain: Terrain::Grass,
            vertices: (
                Point3 {
                    x: -1000,
                    y: 0,
                    z: 1000,
                },
                Point3 {
                    x: 1000,
                    y: 0,
                    z: -1000,
                },
                Point3 {
                    x: -1000,
                    y: 0,
                    z: -1000,
                },
            ),
        },
    ]*/

    triangles
}

pub fn mario_to_cpumesh(geo: &MarioGeometry) -> CpuMesh {
    let positions = geo
        .positions()
        .iter()
        .map(|p| vec3(p.x / 100., p.y / 100., p.z / 100.))
        .collect();

    let colors = geo
        .colors()
        .iter()
        .map(|c| Srgba::new_opaque((c.r * 255.) as u8, (c.g * 255.) as u8, (c.b * 255.) as u8))
        .collect();

    let uvs = geo.uvs().iter().map(|u| vec2(u.x, u.y)).collect();

    let normals = geo.normals().iter().map(|n| vec3(n.x, n.y, n.z)).collect();

    CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        uvs: Some(uvs),
        normals: Some(normals),
        ..Default::default()
    }
}

pub fn make_mat(context: &three_d::Context, raw_texture: Texture) -> MarioMaterial {
    let data = raw_texture.data;
    let len = (raw_texture.width * raw_texture.height) as usize;
    let mut texture_data = Vec::with_capacity(len);
    // TODO: make this cleaner
    for i in 0..len {
        let idx = i * 4;
        texture_data.push([data[idx], data[idx + 1], data[idx + 2], data[idx + 3]]);
    }

    let cpu_texture = CpuTexture {
        name: "Mario Mario".to_owned(),
        data: TextureData::RgbaU8(texture_data),
        width: raw_texture.width,
        height: raw_texture.height,
        min_filter: Interpolation::Linear,
        mag_filter: Interpolation::Linear,
        mipmap: None,
        wrap_s: Wrapping::Repeat,
        wrap_t: Wrapping::Repeat,
    };

    let texture = Texture2DRef::from_cpu_texture(context, &cpu_texture);

    MarioMaterial {
        texture,
        render_states: RenderStates::default(),
    }
}
