#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform sampler2D DEPTH;
uniform sampler2D NOISE;

uniform vec2 RES;
uniform float FRAME;
uniform float SPEED = 0.1;

void main() {
    float time = FRAME * SPEED;
    vec4 color = texture(COLOR, fUv);

    float s = (sin(time) + 1.0) * 0.5;
    vec2 noiseUv = vec2(fUv.x + s, fUv.y + time) * 2.0;
    noiseUv.x = mod(noiseUv.x, 1.0);
    noiseUv.y = mod(noiseUv.y, 1.0);
    vec4 noise = texture(NOISE, noiseUv);
    vec4 changed = color;
    changed.a = 1.0 - noise.r * 0.6;
    outColor = mix(color, changed, s);
}