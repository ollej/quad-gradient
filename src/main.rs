#![windows_subsystem = "windows"]
use macroquad::prelude::*;

#[macroquad::main("Gradient")]
async fn main() {
    let texture = render_target(500, 500).texture;
    let material = load_material(
        ShaderSource {
            glsl_vertex: Some(GRADIENT_VERTEX_SHADER),
            glsl_fragment: Some(GRADIENT_FRAGMENT_SHADER),
            metal_shader: None,
        },
        MaterialParams {
            uniforms: vec![("canvasSize".to_owned(), UniformType::Float2)],
            ..Default::default()
        },
    )
    .unwrap();

    set_default_camera();
    loop {
        clear_background(WHITE);
        gl_use_material(&material);
        material.set_uniform("canvasSize", (screen_width(), screen_height()));
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await;
    }
}

const GRADIENT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
uniform vec2 canvasSize;
uniform vec4 _Time;
uniform sampler2D Texture;

void main() {
    float time = _Time.x;
    vec2 coord = gl_FragCoord.xy / canvasSize.xy; // [0,1]
    coord = 2. * (coord - 1.); // [-1,1]
    float scale = sin(time) + 1.;
    coord = coord / scale;
    if (abs(coord.x) < 1. && abs(coord.y) < 1.) {
        coord = (coord + 1.) / 2.;
        gl_FragColor = vec4(coord.x, coord.y, 1. - coord.x, 1);
    } else {
        gl_FragColor = vec4(1,1,1,1); // white
    }
}
"#;

const GRADIENT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
}
";
