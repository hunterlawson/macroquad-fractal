#version 100
precision highp float;

uniform float base_re;
uniform float scale_re;
uniform float base_im;
uniform float scale_im;
uniform vec2 c;

uniform int max_iter;

const int MAX_ITER = 1000;

// the output from the vertex shader
varying highp vec2 uv;

void main() {
    vec2 z = vec2(base_re + uv.x * scale_re, base_im + uv.y * scale_im);
    float z_x2 = z.x * z.x;
    float z_y2 = z.y * z.y;

    for(int i = 0; i < MAX_ITER; i++) {
        if (i >= max_iter ) break;
        if (z_x2 + z_y2 > 4.) {
            gl_FragColor = vec4(1., 0., 0., 1.);
            return;
        }
        z = vec2(z_x2 - z_y2, 2. * z.x * z.y) + c;
        z_x2 = z.x * z.x;
        z_y2 = z.y * z.y;
    }

    gl_FragColor = vec4(0., 0., 0., 1.);
}