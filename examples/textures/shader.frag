#version 330 core

uniform sampler2D awesomefaceTexture;

in vec3 vertexColor;
in vec2 textureCoordinates;
out vec4 FragColor;

void main() {
    FragColor = texture(awesomefaceTexture, textureCoordinates);
}