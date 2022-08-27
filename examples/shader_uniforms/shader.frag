#version 330 core

out vec4 FragColor;

uniform float triangleBrightness;

void main() {
    FragColor = triangleBrightness * vec4(0.8, 0.3, 0.5, 1.0);
}