#version 450

layout (location = 0) in vec2 fUv;
layout (location = 0) out vec4 outColor;

uniform sampler2D COLOR;
uniform sampler2D DEPTH;
uniform vec2 RES;
uniform vec2 CAM;
uniform vec2 DIRECTION = vec2(1.0, 0.0);
uniform float FRAME;

const float FACTOR = 150;
const float EFFECT = 0.15;

vec2 hash( vec2 p ) {
    p = vec2(dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)));
    return -1.0 + 2.0*fract(sin(p)*43758.5453123);
}


float noise( in vec2 p ) {
    const float K1 = 0.366025404; // (sqrt(3)-1)/2;
    const float K2 = 0.211324865; // (3-sqrt(3))/6;
    vec2 i = floor(p + (p.x+p.y)*K1);
    vec2 a = p - i + (i.x+i.y)*K2;
    vec2 o = (a.x>a.y) ? vec2(1.0,0.0) : vec2(0.0,1.0); //vec2 of = 0.5 + 0.5*vec2(sign(a.x-a.y), sign(a.y-a.x));
    vec2 b = a - o + K2;
    vec2 c = a - 1.0 + 2.0*K2;
    vec3 h = max(0.5-vec3(dot(a,a), dot(b,b), dot(c,c) ), 0.0 );
    vec3 n = h*h*h*h*vec3( dot(a,hash(i+0.0)), dot(b,hash(i+o)), dot(c,hash(i+1.0)));
    return dot(n, vec3(70.0));
}

void main() {
    vec2 pos = CAM + (FRAME * FACTOR * DIRECTION);
    vec2 uv = (fUv - pos / RES) * RES / FACTOR;
    float noise = (noise(uv) / 7.0 + noise(uv / 1.5) / 3.0 + noise(uv / 5.0) * 1.2) * EFFECT + (1.0 - EFFECT);
    //float noise = noise((fUv - CAM / RES) * FACTOR) * EFFECT + (1.0 - EFFECT);
    vec4 color = texture(COLOR, fUv);
    outColor = vec4(color.rgb * noise, 1.0);
}