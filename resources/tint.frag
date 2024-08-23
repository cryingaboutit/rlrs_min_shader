# version 120

uniform sampler2D in_texture;
uniform vec2 in_resolution;
uniform float in_x;
uniform float in_y;

vec4 get_texel(sampler2D tex, vec2 normalized_pos) {
	return texture2D(tex, normalized_pos);
}

void main() {
    vec2 normalized_pos = gl_FragCoord.xy / in_resolution.xy;
    vec4 texel = get_texel(in_texture, normalized_pos);
    gl_FragColor = texel + vec4(normalized_pos.x, normalized_pos.y, 0, 0.5);
}