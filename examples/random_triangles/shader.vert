#version 330 core

layout (location = 0) in vec3 lVertexPosition;
layout (location = 1) in vec3 lVertexColor;
layout (location = 2) in vec2 lVertexTextureCoordinates;

out vec3 vertexColor;

void main() {
    vertexColor = lVertexColor;

    gl_Position = vec4(lVertexPosition, 1.0);
}