#version 330 core


uniform sampler2D ourTexture;

in VS_OUTPUT {
  vec3 Color;
  vec2 TexCoord;
} IN;

out vec4 Color;

void main()
{
  Color = vec4(IN.Color, 1.0f);
  fragColor = texture(ourTexture,TexCoord);
}