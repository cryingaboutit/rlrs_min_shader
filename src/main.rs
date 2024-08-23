use raylib::prelude::*;


enum Mode {
    PostProcessing,
    RegularCamera,
    DrawCanvas,
}

fn main() {
    //change mode here
    let mode = Mode::PostProcessing;

    let (mut h_r, mut th_r) = init()
        .size(640, 480)
        .title("ඞ")
        .build();

    // load shader
    let mut shader_0 = h_r.load_shader(&th_r, None, Some("resources/texel_coord_float.frag")).expect("failed to load .frag");
    let s0_loc_texture = shader_0.get_shader_location("in_texture");
    let s0_loc_resolution = shader_0.get_shader_location("in_resolution");
    let s0_loc_x = shader_0.get_shader_location("in_x");
    let s0_loc_y = shader_0.get_shader_location("in_y");

    // load image
    let mut rect_scarfy = Rectangle::new(0., 0., 768. / 6., 128.);
    let mut tex_scarfy = h_r.load_texture(&th_r, "./resources/scarfy.png").expect("failed to find scarfy");
    let mut dest_scarfy = Rectangle::new(320., 240., 768. / 6., 128. / 2.);
    let mut origin_scarfy = Vector2::new(768. / 12., 128. / 2.);

    // plain canvas to be drawn on
    let mut canvas = h_r.load_render_texture(&th_r, 640, 480).expect("failed to create canvas");

    // camera
    let mut c = Camera2D::default();
    c.zoom = 1.;

    while !h_r.window_should_close() {
        match mode {
            Mode::RegularCamera => regular_camera(&mut h_r, &mut th_r, &mut c, &tex_scarfy),
            Mode::DrawCanvas => draw_canvas_onto_screen(&mut h_r, &mut th_r, &mut c, &mut tex_scarfy, &mut canvas),
            Mode::PostProcessing => post_processing(&mut h_r, &mut th_r, &mut shader_0, s0_loc_texture, s0_loc_resolution, s0_loc_x, s0_loc_y, &mut c, &mut tex_scarfy, &mut canvas),
            _ => continue,
        }
    }
}


fn post_processing(
    h_r: &mut RaylibHandle,
    th_r: &mut RaylibThread,
    shader: &mut Shader,
    loc_texture: i32,
    loc_resolution: i32,
    loc_x: i32,
    loc_y: i32,
    c: &mut Camera2D,
    tex: &mut Texture2D,
    canvas: &mut RenderTexture2D,
) {
    let mut handle_draw = h_r.begin_drawing(&th_r);

    // draw to canvas
    {
        let mut handle_texture = handle_draw.begin_texture_mode(&th_r, canvas);

        handle_texture.clear_background(Color::WHEAT);

        let mut handle_camera = handle_texture.begin_mode2D(*c);

        handle_camera.draw_texture(tex, 0, 0, Color::WHITE);
    }

    // set shader value
    shader.set_shader_value_texture(loc_texture, canvas.texture());
    shader.set_shader_value_v(loc_resolution, &[canvas.width() as f32, canvas.height() as f32]);
    shader.set_shader_value(loc_x, canvas.width() as f32);
    shader.set_shader_value(loc_y, canvas.height() as f32);

    // draw to screen
    {
        let mut handle_shader = handle_draw.begin_shader_mode(shader);

        handle_shader.draw_texture_rec(
            &canvas, 
            Rectangle::new(0., 0., canvas.texture.width as f32, -canvas.texture.height as f32), 
            Vector2::zero(), 
            Color::WHITE
        );
    }
}

fn regular_camera(
    h_r: &mut RaylibHandle,
    th_r: &mut RaylibThread,
    c: &mut Camera2D,
    tex: &Texture2D
) {
    let mut handle_draw = h_r.begin_drawing(&th_r);
    {
        let mut handle_camera = (&mut handle_draw).begin_mode2D(*c);
        handle_camera.draw_texture(tex, 0, 0, Color::WHITE);

    }
}

fn draw_canvas_onto_screen(
    h_r: &mut RaylibHandle,
    th_r: &mut RaylibThread,
    c: &mut Camera2D,
    tex: &mut Texture2D,
    canvas: &mut RenderTexture2D,
) {
    let mut handle_draw = h_r.begin_drawing(&th_r);
    {
        let mut handle_texture = handle_draw.begin_texture_mode(&th_r, canvas);

        handle_texture.clear_background(Color::WHEAT);

        let mut handle_camera = handle_texture.begin_mode2D(*c);

        handle_camera.draw_texture(tex, 0, 0, Color::WHITE);
    }
    {
        handle_draw.draw_texture_rec(
            &canvas, 
            Rectangle::new(0., 0., canvas.texture.width as f32, -canvas.texture.height as f32), 
            Vector2::zero(), 
            Color::WHITE
        )
    }
}