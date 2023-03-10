#version 100
precision mediump float;
void main() {
  vec2 fragmentPosition = 2.0 * gl_PointCoord - 1.0;
  float distance = length(fragmentPosition);
  float distanceSqrd = distance * distance;
  gl_FragColor = vec4(
                      0.2 / distanceSqrd,
                      0.1 / distanceSqrd,
                      0.0, 1.0 );
}
