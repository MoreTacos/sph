#version 450

layout(location=0) in vec2 a_position;

layout(location=5) in mat4 model_matrix;
layout(location=9) in vec2 scale;

void main() {
    gl_Position = model_matrix * vec4(a_position.x * scale.x, a_position.y * scale.y, 0.0, 1.0);
}
