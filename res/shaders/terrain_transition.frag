#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;   // scene color + seam mask
uniform vec2 RES;          // resolution in pixels
uniform float SIZE;        // seam offset in pixels

void main() {
    vec4 col = texture(COLOR, fUv);

    const vec3 RED   = vec3(1.0, 0.0, 0.0); // vertical seam
    const vec3 GREEN = vec3(0.0, 1.0, 0.0); // horizontal seam
    float eps = 0.01;

    vec2 pixelPos = fUv * RES;
    vec4 finalColor = col;

    if (distance(col.rgb, RED) < eps) {
        // Vertical seam → hop along Y
        float w = sin(pixelPos.x * 0.5);
        float m = (sin(pixelPos.x * 0.7) + 1.0) * 0.5;
        m *= SIZE;
        float dir = (w < 0.0) ? -1.0 : 1.0;
        vec2 shiftedUV = fUv + vec2(0.0, dir * SIZE * 2.0 + dir * m) / RES;
        finalColor = texture(COLOR, shiftedUV);

    } else if (distance(col.rgb, GREEN) < eps) {
        // Horizontal seam → hop along X
        float w = sin(pixelPos.y * 0.5);
        float m = (sin(pixelPos.y * 0.7) + 1.0) * 0.5;
        m *= SIZE;
        float dir = (w < 0.0) ? -1.0 : 1.0;
        vec2 shiftedUV = fUv + vec2(dir * SIZE * 2.0 + dir * m, 0.0) / RES;
        finalColor = texture(COLOR, shiftedUV);
    }

    outColor = finalColor;
}
