#version 100
precision highp float;

uniform float base_re;
uniform float scale_re;
uniform float base_im;
uniform float scale_im;
uniform vec2 c;

uniform int max_iter;

// the output from the vertex shader
varying highp vec2 uv;

void main() {
    vec2 z = vec2(base_re + uv.x * scale_re, base_im + uv.y * scale_im);

    for(int i = 0; i < 100; i++) {
        if (z.x * z.x + z.y * z.y > 4.) {
            gl_FragColor = vec4(1., 0., 0., 1.);
            return;
        }
        z = vec2(z.x * z.x - z.y * z.y, 2. * z.x * z.y) + c;
    }

    gl_FragColor = vec4(0., 0., 0., 1.);
}