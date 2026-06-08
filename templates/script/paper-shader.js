/**
 * Paper texture shader adapted from @paper-design/shaders
 * https://github.com/paper-design/shaders
 *
 * Licensed under the PolyForm Shield License 1.0.0
 * https://polyformproject.org/licenses/shield/1.0.0
 */
(() => {
  "use strict";

  const VERTEX_SHADER = `#version 300 es
precision mediump float;

in vec2 a_position;

uniform vec2 u_resolution;
uniform float u_pixelRatio;
uniform float u_imageAspectRatio;

out vec2 v_imageUV;

void main() {
  gl_Position = vec4(a_position, 0.0, 1.0);
  v_imageUV = a_position * 0.5 + 0.5;
}
`;

  const FRAGMENT_SHADER = `#version 300 es
precision mediump float;

uniform vec2 u_resolution;
uniform float u_pixelRatio;

uniform vec4 u_colorFront;
uniform vec4 u_colorBack;

uniform float u_contrast;
uniform float u_roughness;
uniform float u_fiber;
uniform float u_fiberSize;
uniform float u_crumples;
uniform float u_crumpleSize;
uniform float u_folds;
uniform float u_foldCount;
uniform float u_drops;
uniform float u_seed;
uniform float u_fade;
uniform float u_imageAspectRatio;

uniform sampler2D u_noiseTexture;

in vec2 v_imageUV;

out vec4 fragColor;

#define PI 3.14159265358979323846
#define TWO_PI 6.28318530717958647692

mat2 rotate(float a) {
  float s = sin(a);
  float c = cos(a);
  return mat2(c, -s, s, c);
}

float randomR(vec2 p) {
  vec2 uv = floor(p) / 50.0 + 0.5;
  return texture(u_noiseTexture, fract(uv)).r;
}

float valueNoise(vec2 st) {
  vec2 i = floor(st);
  vec2 f = fract(st);
  float a = randomR(i);
  float b = randomR(i + vec2(1.0, 0.0));
  float c = randomR(i + vec2(0.0, 1.0));
  float d = randomR(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  float x1 = mix(a, b, u.x);
  float x2 = mix(c, d, u.x);
  return mix(x1, x2, u.y);
}

float fbm(vec2 n) {
  float total = 0.0, amplitude = 0.4;
  for (int i = 0; i < 3; i++) {
    total += valueNoise(n) * amplitude;
    n *= 1.99;
    amplitude *= 0.65;
  }
  return total;
}

float randomG(vec2 p) {
  vec2 uv = floor(p) / 50.0 + 0.5;
  return texture(u_noiseTexture, fract(uv)).g;
}

float roughness(vec2 p) {
  p *= 0.1;
  float o = 0.0;
  for (float i = 0.0; ++i < 4.0; p *= 2.1) {
    vec4 w = vec4(floor(p), ceil(p));
    vec2 f = fract(p);
    o += mix(
      mix(randomG(w.xy), randomG(w.xw), f.y),
      mix(randomG(w.zy), randomG(w.zw), f.y),
      f.x);
    o += 0.2 / exp(2.0 * abs(sin(0.2 * p.x + 0.5 * p.y)));
  }
  return o / 3.0;
}

float fiberNoise(vec2 uv, vec2 offset) {
  float t = 0.0;
  for (float i = 1.0; i <= 3.0; i += 1.0) {
    float power = pow(2.0, i);
    float invPower = 1.0 / power;
    vec2 p = uv * power + offset;
    float angle = randomR(floor(p)) * TWO_PI;
    vec2 dir = vec2(cos(angle), sin(angle));
    vec2 f = fract(p) - 0.5;
    t += abs(dot(f, dir)) * invPower;
  }
  return t * 2.0;
}

vec2 randomGB(vec2 p) {
  vec2 uv = floor(p) / 50.0 + 0.5;
  return texture(u_noiseTexture, fract(uv)).gb;
}

float crumpledNoise(vec2 t, float pw) {
  vec2 p = floor(t);
  float wsum = 0.0;
  float cl = 0.0;
  for (int y = -1; y < 2; y += 1) {
    for (int x = -1; x < 2; x += 1) {
      vec2 b = vec2(float(x), float(y));
      vec2 q = b + p;
      vec2 q2 = q - floor(q / 8.0) * 8.0;
      vec2 c = q + randomGB(q2);
      vec2 r = c - t;
      float w = pow(smoothstep(0.0, 1.0, 1.0 - abs(r.x)), pw)
              * pow(smoothstep(0.0, 1.0, 1.0 - abs(r.y)), pw);
      cl += (0.5 + 0.5 * sin((q2.x + q2.y * 5.0) * 8.0)) * w;
      wsum += w;
    }
  }
  return pow(wsum != 0.0 ? cl / wsum : 0.0, 0.5) * 2.0;
}

float crumplesShape(vec2 uv) {
  return crumpledNoise(uv * 0.25, 16.0) * crumpledNoise(uv * 0.5, 2.0);
}

vec2 folds(vec2 uv) {
  vec3 pp = vec3(0.0);
  float l = 9.0;
  for (float i = 0.0; i < 15.0; i++) {
    if (i >= u_foldCount) break;
    vec2 rand = randomGB(vec2(i, i * u_seed));
    float an = rand.x * TWO_PI;
    vec2 p = vec2(cos(an), sin(an)) * rand.y;
    float dist = distance(uv, p);
    l = min(l, dist);
    if (l == dist) {
      pp.xy = (uv - p.xy);
      pp.z = dist;
    }
  }
  return mix(pp.xy, vec2(0.0), pow(pp.z, 0.25));
}

float drops(vec2 uv) {
  vec2 iDropsUV = floor(uv);
  vec2 fDropsUV = fract(uv);
  float dropsMinDist = 1.0;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 neighbor = vec2(float(i), float(j));
      vec2 offset = randomGB(iDropsUV + neighbor);
      offset = 0.5 + 0.5 * sin(10.0 * u_seed + TWO_PI * offset);
      vec2 pos = neighbor + offset - fDropsUV;
      float dist = length(pos);
      dropsMinDist = min(dropsMinDist, dropsMinDist * dist);
    }
  }
  return 1.0 - smoothstep(0.05, 0.09, pow(dropsMinDist, 0.5));
}

void main() {
  vec2 patternUV = v_imageUV - 0.5;
  patternUV = 5.0 * (patternUV * vec2(u_imageAspectRatio, 1.0));

  vec2 roughnessUv = 1.5 * (gl_FragCoord.xy - 0.5 * u_resolution) / u_pixelRatio;
  float rough = roughness(roughnessUv + vec2(1.0, 0.0)) - roughness(roughnessUv - vec2(1.0, 0.0));

  vec2 crumplesUV = fract(patternUV * 0.02 / u_crumpleSize - u_seed) * 32.0;
  float crumple = u_crumples * (crumplesShape(crumplesUV + vec2(0.05, 0.0)) - crumplesShape(crumplesUV));

  vec2 fiberUV = 2.0 / u_fiberSize * patternUV;
  float fib = fiberNoise(fiberUV, vec2(0.0));
  fib = 0.5 * u_fiber * (fib - 1.0);

  vec2 normal = vec2(0.0);

  vec2 foldsUV = patternUV * 0.12;
  foldsUV = rotate(4.0 * u_seed) * foldsUV;
  vec2 w = folds(foldsUV);
  foldsUV = rotate(0.01 * sin(u_seed)) * (foldsUV + 0.007 * cos(u_seed));
  vec2 w2 = folds(foldsUV);

  float drop = u_drops * drops(patternUV * 2.0);

  float fade = u_fade * fbm(0.17 * patternUV + 10.0 * u_seed);
  fade = clamp(8.0 * fade * fade * fade, 0.0, 1.0);

  w = mix(w, vec2(0.0), fade);
  w2 = mix(w2, vec2(0.0), fade);
  crumple = mix(crumple, 0.0, fade);
  drop = mix(drop, 0.0, fade);
  fib *= mix(1.0, 0.5, fade);
  rough *= mix(1.0, 0.5, fade);

  normal.xy += u_folds * min(5.0 * u_contrast, 1.0) * 4.0 * max(vec2(0.0), w + w2);
  normal.xy += crumple;
  normal.xy += 3.0 * drop;
  normal.xy += u_roughness * 1.5 * rough;
  normal.xy += fib;

  vec3 lightPos = vec3(1.0, 2.0, 1.0);
  float res = dot(normalize(vec3(normal, 9.5 - 9.0 * pow(u_contrast, 0.1))), normalize(lightPos));

  vec3 fgColor = u_colorFront.rgb * u_colorFront.a;
  float fgOpacity = u_colorFront.a;
  vec3 bgColor = u_colorBack.rgb * u_colorBack.a;
  float bgOpacity = u_colorBack.a;

  vec3 color = fgColor * res;
  float opacity = fgOpacity * res;

  color += bgColor * (1.0 - opacity);
  opacity += bgOpacity * (1.0 - opacity);

  color -= 0.007 * drop;

  fragColor = vec4(color, opacity);
}
`;

  function createNoiseTexture(gl, size) {
    const data = new Uint8Array(size * size * 4);
    for (let i = 0; i < size * size; i++) {
      data[i * 4] = Math.random() * 255;
      data[i * 4 + 1] = Math.random() * 255;
      data[i * 4 + 2] = Math.random() * 255;
      data[i * 4 + 3] = 255;
    }
    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, size, size, 0, gl.RGBA, gl.UNSIGNED_BYTE, data);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
    return texture;
  }

  function compileShader(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      console.error("Shader compile error:", gl.getShaderInfoLog(shader));
      gl.deleteShader(shader);
      return null;
    }
    return shader;
  }

  function createProgram(gl, vertSrc, fragSrc) {
    const vert = compileShader(gl, gl.VERTEX_SHADER, vertSrc);
    const frag = compileShader(gl, gl.FRAGMENT_SHADER, fragSrc);
    if (!vert || !frag) return null;

    const program = gl.createProgram();
    gl.attachShader(program, vert);
    gl.attachShader(program, frag);
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error("Program link error:", gl.getProgramInfoLog(program));
      return null;
    }
    return program;
  }

  function initPaperShader(canvas, options = {}) {
    const gl = canvas.getContext("webgl2", { alpha: true, premultipliedAlpha: true });
    if (!gl) {
      console.warn("WebGL2 not supported, skipping paper shader");
      return null;
    }

    const program = createProgram(gl, VERTEX_SHADER, FRAGMENT_SHADER);
    if (!program) return null;

    const posBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, posBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);

    const posAttr = gl.getAttribLocation(program, "a_position");

    const noiseTexture = createNoiseTexture(gl, 256);

    const uniforms = {};
    const uniformNames = [
      "u_resolution", "u_pixelRatio", "u_colorFront", "u_colorBack",
      "u_contrast", "u_roughness", "u_fiber", "u_fiberSize",
      "u_crumples", "u_crumpleSize", "u_folds", "u_foldCount",
      "u_drops", "u_seed", "u_fade", "u_noiseTexture", "u_imageAspectRatio"
    ];
    for (const name of uniformNames) {
      uniforms[name] = gl.getUniformLocation(program, name);
    }

    const colorFront = options.colorFront || [1.0, 1.0, 1.0, 1.0];
    const colorBack = options.colorBack || [0.96, 0.95, 0.92, 0.0];
    const seed = options.seed ?? (Math.random() * 1000);

    function render() {
      const dpr = window.devicePixelRatio || 1;
      const rect = canvas.getBoundingClientRect();
      const w = Math.round(rect.width * dpr);
      const h = Math.round(rect.height * dpr);

      if (canvas.width !== w || canvas.height !== h) {
        canvas.width = w;
        canvas.height = h;
      }

      gl.viewport(0, 0, w, h);
      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);

      gl.useProgram(program);

      gl.enableVertexAttribArray(posAttr);
      gl.bindBuffer(gl.ARRAY_BUFFER, posBuffer);
      gl.vertexAttribPointer(posAttr, 2, gl.FLOAT, false, 0, 0);

      gl.uniform2f(uniforms.u_resolution, w, h);
      gl.uniform1f(uniforms.u_pixelRatio, dpr);
      gl.uniform1f(uniforms.u_imageAspectRatio, rect.width / rect.height);

      gl.uniform4fv(uniforms.u_colorFront, colorFront);
      gl.uniform4fv(uniforms.u_colorBack, colorBack);

      gl.uniform1f(uniforms.u_contrast, options.contrast ?? 0.35);
      gl.uniform1f(uniforms.u_roughness, options.roughness ?? 0.52);
      gl.uniform1f(uniforms.u_fiber, options.fiber ?? 0.5);
      gl.uniform1f(uniforms.u_fiberSize, options.fiberSize ?? 0.4);
      gl.uniform1f(uniforms.u_crumples, options.crumples ?? 0.15);
      gl.uniform1f(uniforms.u_crumpleSize, options.crumpleSize ?? 0.45);
      gl.uniform1f(uniforms.u_folds, options.folds ?? 0.08);
      gl.uniform1f(uniforms.u_foldCount, options.foldCount ?? 4.0);
      gl.uniform1f(uniforms.u_drops, options.drops ?? 0.0);
      gl.uniform1f(uniforms.u_seed, seed);
      gl.uniform1f(uniforms.u_fade, options.fade ?? 0.0);

      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, noiseTexture);
      gl.uniform1i(uniforms.u_noiseTexture, 0);

      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    }

    render();
    return { render, gl, canvas };
  }

  function applyPaperShaders() {
    const cards = document.querySelectorAll(".paper-card, .torn-paper-front");
    const instances = [];

    cards.forEach((card, index) => {
      if (card.querySelector(".paper-shader-canvas")) return;

      const canvas = document.createElement("canvas");
      canvas.classList.add("paper-shader-canvas");
      canvas.setAttribute("aria-hidden", "true");
      canvas.setAttribute("role", "presentation");
      card.appendChild(canvas);

      const isCodeBlock = card.classList.contains("torn-paper-front");
      const shaderOptions = isCodeBlock ? {
        colorFront: [0.808, 0.831, 0.851, 0.15],
        colorBack: [1.0, 1.0, 1.0, 0.0],
        contrast: 0.12,
        roughness: 0.20,
        fiber: 0.15,
        fiberSize: 0.15,
        crumples: 0.12,
        crumpleSize: 0.35,
        folds: 0.15,
        foldCount: 3,
        drops: 0.0,
        fade: 0.00,
        seed: 12
      } : {
        colorFront: [0.808, 0.831, 0.851, 1.0],
        colorBack: [1.0, 1.0, 1.0, 1.0],
        contrast: 0.30,
        roughness: 0.40,
        fiber: 0.30,
        fiberSize: 0.20,
        crumples: 0.30,
        crumpleSize: 0.35,
        folds: 0.65,
        foldCount: 5,
        drops: 0.20,
        fade: 0.00,
        seed: 6
      };

      const instance = initPaperShader(canvas, shaderOptions);

      if (instance) instances.push(instance);
    });

    let resizeTimer;
    window.addEventListener("resize", () => {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => {
        instances.forEach((inst) => inst.render());
      }, 100);
    });

    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const canvas = entry.target.querySelector(".paper-shader-canvas");
        if (!canvas) continue;
        const inst = instances.find((i) => i.canvas === canvas);
        if (inst) inst.render();
      }
    });

    cards.forEach((card) => resizeObserver.observe(card));
    document.body.setAttribute("data-shaders-ready", "true");
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", applyPaperShaders);
  } else {
    applyPaperShaders();
  }
})();
