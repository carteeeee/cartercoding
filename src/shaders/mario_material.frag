uniform sampler2D tex;
uniform vec3 cameraPosition;
uniform mat3 textureTransformation;

in vec3 pos;
in vec3 nor;
in vec4 col;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main() {
  vec4 texcol = texture(tex, (textureTransformation * vec3(uvs, 1.0)).xy);
  vec4 surfaceColor;
  if (texcol.a > 0.) {
    surfaceColor = mix(col, texcol, pow(texcol.a, 2.0));
  } else {
    surfaceColor = col;
  }

  vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
  float metallic_factor = 0.0;
  float roughness_factor = 1.0;
  float occlusion = 1.0;


  outColor.rgb = calculate_lighting(cameraPosition, surfaceColor.rgb, pos, normal, metallic_factor, roughness_factor, occlusion);
  outColor.rgb = tone_mapping(outColor.rgb);
  outColor.rgb = color_mapping(outColor.rgb);
  outColor.a = surfaceColor.a;
}
