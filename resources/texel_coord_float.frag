# version 120

uniform sampler2D in_texture;
uniform vec2 in_resolution;
uniform float in_x;
uniform float in_y;

void main() {
    vec2 normalized_pos = vec2(gl_FragCoord.x / in_x, gl_FragCoord.y / in_y);
    gl_FragColor = vec4(normalized_pos.x, normalized_pos.y, 0., 1.);
}