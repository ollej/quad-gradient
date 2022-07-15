use macroquad::prelude::*;

#[macroquad::main("Gradient")]
async fn main() {
    let render_target = render_target(640, 480);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        GRADIENT_VERTEX_SHADER,
        GRADIENT_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![("canvasSize".to_owned(), UniformType::Float2)],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        // drawing to the texture

        // 0..100, 0..100 camera
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target),
            ..Default::default()
        });

        // drawing to the screen

        set_default_camera();

        clear_background(WHITE);
        gl_use_material(material);
        material.set_uniform("canvasSize", (screen_width(), screen_height()));
        draw_texture_ex(
            render_target.texture,
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

uniform sampler2D Texture;

void main() {
    gl_FragColor = vec4(gl_FragCoord.x/canvasSize.x, 0, 0, 1);
}
"#;

const GRADIENT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec2 canvasSize;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
}
";
