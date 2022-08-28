#version 330 core

in vec3 vertexColor;
in vec2 textureCoordinates;

out vec4 FragColor;

void main() {
    FragColor = vec4(vertexColor, 1.0f);
}