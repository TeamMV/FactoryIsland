#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform sampler2D DEPTH;
uniform vec2 RES;

uniform float FRAME;
uniform float SPEED = 0.1;

void main() {
    vec4 color = texture(COLOR, fUv);

    float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));

    float mask = step(0.001, gray);

    //vec3 tint = vec3(1.0, 1.0, 1.0);
    vec3 base = color.rgb;

    vec3 hologramRGB = base;

    float alpha = mix(0.8, 1.0, (sin(FRAME * SPEED) * 0.5 + 0.5)) * mask;

    outColor = vec4(hologramRGB, alpha);
}