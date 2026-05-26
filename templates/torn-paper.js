(function() {
  "use strict";

  function generateNoise(steps, amplitude, offset) {
    const values1 = [];
    const values2 = [];
    for (let i = 0; i < steps; i++) {
      values1.push(Math.random() - 0.5);
      values2.push(Math.random() - 0.5);
    }

    const smoothed = [];
    const win1 = 5;
    const win2 = 2;

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
      if (W === 0) return;

      const step = 8;
      const steps = Math.ceil(W / step) + 1;

      const topNoise = generateNoise(steps, 8, 12);
      const bottomNoise = generateNoise(steps, 8, 12);

      const topPoints = [];
      topPoints.push("-2px -2px");
      topPoints.push(`${W + 2}px -2px`);
      topPoints.push(`${W + 2}px ${topNoise[steps - 1]}px`);
      for (let i = steps - 1; i >= 0; i--) {
        const x = Math.min(i * step, W);
        topPoints.push(`${x}px ${topNoise[i]}px`);
      }
      topPoints.push(`-2px ${topNoise[0]}px`);
      const topClip = `polygon(${topPoints.join(", ")})`;

      const bottomPoints = [];
      bottomPoints.push(`-2px ${bottomNoise[0]}px`);
      for (let i = 0; i < steps; i++) {
        const x = Math.min(i * step, W);
        bottomPoints.push(`${x}px ${bottomNoise[i]}px`);
      }
      bottomPoints.push(`${W + 2}px ${bottomNoise[steps - 1]}px`);
      bottomPoints.push(`${W + 2}px 26px`);
      bottomPoints.push(`-2px 26px`);
      const bottomClip = `polygon(${bottomPoints.join(", ")})`;

      const topEdge = container.querySelector(".torn-edge-top");
      const bottomEdge = container.querySelector(".torn-edge-bottom");

      if (topEdge) topEdge.style.clipPath = topClip;
      if (bottomEdge) bottomEdge.style.clipPath = bottomClip;
    });

    const truncatedCards = document.querySelectorAll(".is-truncated");
    truncatedCards.forEach((card) => {
      const W = card.offsetWidth;
      if (W === 0) return;

      const step = 6;
      const steps = Math.ceil(W / step) + 1;

      const noise = generateNoise(steps, 10, 28);

      const points = [];
      points.push("0% 0%");
      points.push("100% 0%");
      points.push(`100% calc(100% - ${noise[steps - 1]}px)`);
      for (let i = steps - 1; i >= 0; i--) {
        const x = Math.min(i * step, W);
        points.push(`${x}px calc(100% - ${noise[i]}px)`);
      }
      points.push(`0px calc(100% - ${noise[0]}px)`);

      card.style.clipPath = `polygon(${points.join(", ")})`;
    });
  }

  function initTears() {
    updateTears();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initTears);
  } else {
    initTears();
  }
  window.addEventListener("load", initTears);

  let resizeTimer;
  window.addEventListener("resize", () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      window.requestAnimationFrame(updateTears);
    }, 50);
  });
})();
