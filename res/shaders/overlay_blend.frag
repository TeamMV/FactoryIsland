#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D SRC;
uniform sampler2D DST;

//custom blend shader for the overlay.
void main() {
    vec4 dst = texture(DST, fUv);
    vec4 src = texture(SRC, fUv);

    float a = sign(max(max(src.r, src.g), src.b));
    vec4 new_rgb = mix(dst.rgba, src.rgba, a);
    outColor = new_rgb;
}
