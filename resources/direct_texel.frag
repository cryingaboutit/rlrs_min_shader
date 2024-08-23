# version 120

uniform sampler2D in_texture;
uniform vec2 in_resolution;

vec4 get_texel(sampler2D tex, vec2 normalized_pos) {
	return texture2D(tex, normalized_pos);
}


void main() {
    vec2 normalized_pos = gl_FragCoord.xy / in_resolution.xy;
    gl_FragColor = get_texel(in_texture, normalized_pos);
}