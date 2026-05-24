(function() {
  "use strict";

  // Procedural Fractal Noise Generator (Low frequency + High frequency)
  function generateNoise(steps, amplitude, offset) {
    const values1 = [];
    const values2 = [];
    for (let i = 0; i < steps; i++) {
      values1.push(Math.random() - 0.5);
      values2.push(Math.random() - 0.5);
    }

    const smoothed = [];
    const win1 = 5; // Low-frequency window for main tear waves
    const win2 = 2; // High-frequency window for small fibrous details

    for (let i = 0; i < steps; i++) {
      let sum1 = 0;
      let count1 = 0;
      for (let w = -win1; w <= win1; w++) {
        const idx = i + w;
        if (idx >= 0 && idx < steps) {
          sum1 += values1[idx];
          count1++;
        }
      }

      let sum2 = 0;
      let count2 = 0;
      for (let w = -win2; w <= win2; w++) {
        const idx = i + w;
        if (idx >= 0 && idx < steps) {
          sum2 += values2[idx];
          count2++;
        }
      }

      const wave1 = (sum1 / count1) * amplitude * 0.75;
      const wave2 = (sum2 / count2) * amplitude * 0.25;
      smoothed.push(offset + wave1 + wave2);
    }
    return smoothed;
  }

  function updateTears() {
    const containers = document.querySelectorAll(".torn-code-container");
    containers.forEach((container) => {
      const W = container.offsetWidth;
      const H = container.offsetHeight;
      if (W === 0 || H === 0) return;

      const step = 8;
      const steps = Math.ceil(W / step) + 1;

      // Base top and bottom offsets
      const topOffset = 10;
      const bottomOffset = H - 10;

      // Generate identical seed noise for both layers so they match shape naturally
      const topNoise = generateNoise(steps, 14, topOffset);
      const bottomNoise = generateNoise(steps, 14, bottomOffset);

      // Back layer coordinates (White paper border)
      const backPoints = [];
      for (let i = 0; i < steps; i++) {
        const x = Math.min(i * step, W);
        backPoints.push(`${x}px ${topNoise[i]}px`);
      }
      backPoints.push(`${W}px ${H / 2}px`);
      for (let i = steps - 1; i >= 0; i--) {
        const x = Math.min(i * step, W);
        backPoints.push(`${x}px ${bottomNoise[i]}px`);
      }
      backPoints.push(`0px ${H / 2}px`);

      // Front layer coordinates (Grid paper, inset by 2px vertically, 3px horizontally)
      const frontPoints = [];
      const insetX = 3;
      const insetY = 2;
      for (let i = 0; i < steps; i++) {
        const pct = i / (steps - 1);
        const x = insetX + pct * (W - 2 * insetX);
        const y = topNoise[i] + insetY;
        frontPoints.push(`${x}px ${y}px`);
      }
      frontPoints.push(`${W - insetX}px ${H / 2}px`);
      for (let i = steps - 1; i >= 0; i--) {
        const pct = i / (steps - 1);
        const x = insetX + pct * (W - 2 * insetX);
        const y = bottomNoise[i] - insetY;
        frontPoints.push(`${x}px ${y}px`);
      }
      frontPoints.push(`${insetX}px ${H / 2}px`);

      const backClip = `polygon(${backPoints.join(", ")})`;
      const frontClip = `polygon(${frontPoints.join(", ")})`;

      const back = container.querySelector(".torn-paper-back");
      const front = container.querySelector(".torn-paper-front");

      if (back) back.style.clipPath = backClip;
      if (front) front.style.clipPath = frontClip;
    });
  }

  function initTears() {
    const containers = document.querySelectorAll(".torn-code-container");
    containers.forEach((container) => {
      if (!container.dataset.initialized) {
        // Random rotation between -0.8deg and 0.8deg for an organic ledger look
        const rot = (Math.random() * 1.6 - 0.8).toFixed(2);
        container.style.transform = `rotate(${rot}deg)`;
        container.dataset.initialized = "true";
      }
    });
    updateTears();
  }

  // Handle initialization on page load
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initTears);
  } else {
    initTears();
  }
  window.addEventListener("load", initTears);

  // Resize listener
  let resizeTimer;
  window.addEventListener("resize", () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      window.requestAnimationFrame(updateTears);
    }, 50);
  });
})();
