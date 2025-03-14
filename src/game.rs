use leptos::prelude::*;
use libsm64::*;
use three_d::{Event, *};

use gloo_timers::callback::Interval;
use log::info;
use std::io::Cursor;
use three_d::Window as ThreeWindow;
use three_d_asset::geometry::Geometry;
use web_sys::HtmlCanvasElement;

use crate::input_match;
use crate::util::*;

const FRAME_TIME: f64 = 100. / 3.;
const UP: Vector3<f32> = vec3(0., 1., 0.);
const CAMERA_SMOOTHING: f32 = 0.5;

macro_rules! pl {
    () => {
        info!("{}", line!())
    };
}

pub async fn run(rom: Vec<u8>, canvas: HtmlCanvasElement) {
    pl!();
    let (mesh, set_mesh) = signal(CpuMesh::square());
    let (target, set_target) = signal(vec3(0., 1.5, 0.));
    let gameinput = RwSignal::new(GameInput::default());
    pl!();

    // in addition to being used for rendering, the glb is used for collision too!
    let mut assets = three_d_asset::io::RawAssets::new();
    assets.insert("level.glb", include_bytes!("../public/level.glb").to_vec());
    let cpu_model: CpuModel = assets.deserialize("level.glb").unwrap();
    pl!();

    let cursor = Cursor::new(rom);
    let mut sm64 = Sm64::new(cursor).unwrap();
    pl!();

    let trimesh;
    if let Geometry::Triangles(t) = &cpu_model.geometries[0].geometry {
        trimesh = t;
    } else {
        panic!("the trimesh is fake now!!");
    }
    let geometry = load_geometry(&trimesh);
    pl!();
    sm64.load_level_geometry(&geometry);
    pl!();

    let mut mario = sm64.create_mario(0., 100., 0.).unwrap();

    pl!();
    let wdw = ThreeWindow::new(WindowSettings {
        title: "mario mario".to_string(),
        max_size: None,
        canvas: Some(canvas),
        ..Default::default()
    })
    .unwrap();
    pl!();

    let context = wdw.gl();

    pl!();
    let mut camera = Camera::new_perspective(
        wdw.viewport(),
        vec3(1.0, 2.0, 0.0),
        target.get_untracked(),
        UP,
        degrees(90.0),
        0.1,
        100.,
    );

    let mut oc = OrbitControl::new(camera.target(), 1.0, 100.);

    let raw_texture = sm64.texture();
    let mat = make_mat(&context, raw_texture);
    let mut model = Gm::new(Mesh::new(&context, &mesh.get_untracked()), mat);

    pl!();
    let level = Model::<PhysicalMaterial>::new(&context, &cpu_model)
        .unwrap()
        .remove(0);
    pl!();

    let light0 = SpotLight::new(
        &context,
        2.0,
        Srgba::WHITE,
        vec3(0., 10., 0.),
        vec3(0., -1., 0.),
        degrees(75.),
        Attenuation::default(),
    );

    let light1 = SpotLight::new(
        &context,
        2.0,
        Srgba::WHITE,
        vec3(10., 20., 15.),
        vec3(0., -1., 0.),
        degrees(75.),
        Attenuation::default(),
    );

    let ambient = AmbientLight::new(&context, 0.1, Srgba::WHITE);

    let _interval = Interval::new(FRAME_TIME as u32, move || {
        let state = mario.tick(gameinput.get_untracked().as_marioinput());
        let pos = state.position;

        let geo = mario.geometry();
        let mario_mesh = mario_to_cpumesh(geo);

        set_target(vec3(pos.x / 100., pos.y / 100., pos.z / 100.));
        set_mesh(mario_mesh);
    });

    let mut last_pos = camera.target();
    let mut target_pos = target.get_untracked() + UP;
    wdw.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);

        for event in frame_input.events.iter() {
            // horible keybinds :3

            if let Event::KeyPress { kind, .. } = event {
                input_match!(
                    [*kind, gameinput, true],
                    [Key::Space, button_a],
                    [Key::Z, button_a],
                    [Key::X, button_b],
                    [Key::C, button_z],
                    [Key::W, stick_up],
                    [Key::S, stick_down],
                    [Key::A, stick_left],
                    [Key::D, stick_right],
                    [Key::ArrowUp, arrow_up],
                    [Key::ArrowDown, arrow_down],
                    [Key::ArrowLeft, arrow_left],
                    [Key::ArrowRight, arrow_right],
                );

                if *kind == Key::J {
                    gameinput.update(|i| i.switch_movement = !i.switch_movement);
                }
            }

            if let Event::KeyRelease { kind, .. } = event {
                input_match!(
                    [*kind, gameinput, false],
                    [Key::Space, button_a],
                    [Key::Z, button_a],
                    [Key::X, button_b],
                    [Key::C, button_z],
                    [Key::W, stick_up],
                    [Key::S, stick_down],
                    [Key::A, stick_left],
                    [Key::D, stick_right],
                    [Key::ArrowUp, arrow_up],
                    [Key::ArrowDown, arrow_down],
                    [Key::ArrowLeft, arrow_left],
                    [Key::ArrowRight, arrow_right],
                )
            }

            if let Event::ModifiersChange { modifiers } = event {
                if modifiers.shift {
                    gameinput.update(|i| i.button_b = true);
                } else {
                    gameinput.update(|i| i.button_b = false);
                }
            }
        }

        let change = (target.get_untracked() + UP) - last_pos;
        target_pos += change * CAMERA_SMOOTHING;
        let cam_pos = camera.position();
        camera.set_view(cam_pos + (target_pos - last_pos), target_pos, UP);
        last_pos = target_pos;
        oc.target = target_pos;
        oc.handle_events(&mut camera, &mut frame_input.events);
        gameinput
            .get_untracked()
            .rotate_camera(&mut camera, target_pos);

        gameinput.update(|i| {
            let d = camera.view_direction();
            i.cam_x = d.x;
            i.cam_z = d.z;
        });

        model.geometry = Mesh::new(&context, &mesh.get_untracked());

        model.set_transformation(Mat4::from_translation(change * -CAMERA_SMOOTHING));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                model.into_iter().chain(&level),
                &[&light0, &light1, &ambient],
            );

        FrameOutput::default()
    });
}
