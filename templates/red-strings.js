(() => {
  "use strict";

  const STRING_COLOR = "#c0392b";
  const STRING_WIDTH = 2.4;
  const LEFT_MARGIN = 28;
  const RIGHT_MARGIN = 28;
  // How much the string droops relative to its span
  const SAG_RATIO = 0.22;
  // Slight horizontal wander for organic look
  const WANDER_PX = 5;
  // How far the string extends into each card (above/below the gap)
  const CARD_PENETRATION = 48;

  /**
   * Build a cubic-bezier "catenary-like" droop path between two points,
   * sagging to the right for left-side strings and to the left for right-side.
   */
  function buildStringPath(x1, y1, x2, y2, sagPx, drift) {
    // Control-point Y sits past the midpoint to create asymmetric droop
    const midY = (y1 + y2) / 2 + sagPx;
    const cp1x = x1 + drift * 0.6;
    const cp2x = x2 - drift * 0.3;
    return `M ${x1} ${y1} C ${cp1x} ${midY}, ${cp2x} ${midY}, ${x2} ${y2}`;
  }

  function createStringSVG(w, h, side, seed) {
    if (h < 8 || w < 40) return null;

    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("width", w);
    svg.setAttribute("height", h);
    svg.setAttribute("viewBox", `0 0 ${w} ${h}`);
    svg.style.position = "absolute";
    svg.style.top = "0";
    svg.style.left = "0";
    svg.style.pointerEvents = "none";
    svg.style.overflow = "visible";
    svg.setAttribute("aria-hidden", "true");

    // Deterministic pseudo-random based on seed
    const prng = () => {
      let x = Math.sin(seed++) * 10000;
      return x - Math.floor(x);
    };

    const sag = Math.max(h * SAG_RATIO, 4);
    const drift = (prng() - 0.5) * 2 * WANDER_PX;

    let x1, x2;
    if (side === "left") {
      x1 = LEFT_MARGIN + (prng() - 0.5) * 6;
      x2 = LEFT_MARGIN + (prng() - 0.5) * 6;
    } else {
      x1 = w - RIGHT_MARGIN + (prng() - 0.5) * 6;
      x2 = w - RIGHT_MARGIN + (prng() - 0.5) * 6;
    }

    const y1 = 0;
    const y2 = h;
    const d = buildStringPath(x1, y1, x2, y2, sag, drift);

    // Shadow layer for depth
    const shadowPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
    shadowPath.setAttribute("d", d);
    shadowPath.setAttribute("fill", "none");
    shadowPath.setAttribute("stroke", "rgba(0,0,0,0.15)");
    shadowPath.setAttribute("stroke-width", STRING_WIDTH + 1.4);
    shadowPath.setAttribute("stroke-linecap", "round");
    shadowPath.style.filter = "blur(1px)";
    shadowPath.style.transform = "translate(0.6px, 1px)";
    svg.appendChild(shadowPath);

    // Main string
    const mainPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
    mainPath.setAttribute("d", d);
    mainPath.setAttribute("fill", "none");
    mainPath.setAttribute("stroke", STRING_COLOR);
    mainPath.setAttribute("stroke-width", STRING_WIDTH);
    mainPath.setAttribute("stroke-linecap", "round");
    svg.appendChild(mainPath);

    // Specular highlight for thread texture
    const highlightPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
    highlightPath.setAttribute("d", d);
    highlightPath.setAttribute("fill", "none");
    highlightPath.setAttribute("stroke", "rgba(230,180,170,0.35)");
    highlightPath.setAttribute("stroke-width", STRING_WIDTH * 0.35);
    highlightPath.setAttribute("stroke-linecap", "round");
    highlightPath.setAttribute("stroke-dasharray", "2 6");
    svg.appendChild(highlightPath);

    return svg;
  }

  function renderStrings() {
    // Remove any previously rendered connectors
    document.querySelectorAll(".red-string-connector").forEach((el) => el.remove());

    const main = document.querySelector("main");
    if (!main) return;

    const cards = main.querySelectorAll(":scope > .article-card-wrapper");
    if (cards.length < 2) return;

    const mainRect = main.getBoundingClientRect();

    for (let i = 0; i < cards.length - 1; i++) {
      const cardA = cards[i];
      const cardB = cards[i + 1];

      const rectA = cardA.getBoundingClientRect();
      const rectB = cardB.getBoundingClientRect();

      // The visible gap between bottom of card A and top of card B (relative to main)
      const gapTopRel = rectA.bottom - mainRect.top;
      const gapBottomRel = rectB.top - mainRect.top;

      // Extend the string into the cards so it looks like it goes behind them
      const containerTop = gapTopRel - CARD_PENETRATION;
      const containerBottom = gapBottomRel + CARD_PENETRATION;
      const containerHeight = containerBottom - containerTop;

      if (containerHeight < 8) continue;

      const containerWidth = mainRect.width;

      // Create the connector container (positioned in main's coordinate space)
      const connector = document.createElement("div");
      connector.className = "red-string-connector";
      connector.style.cssText = `
        position: absolute;
        top: ${containerTop}px;
        left: 0;
        width: ${containerWidth}px;
        height: ${containerHeight}px;
        pointer-events: none;
        z-index: -1;
      `;
      connector.setAttribute("aria-hidden", "true");

      // Left string
      const leftSvg = createStringSVG(containerWidth, containerHeight, "left", (i + 1) * 137);
      if (leftSvg) connector.appendChild(leftSvg);

      // Right string
      const rightSvg = createStringSVG(containerWidth, containerHeight, "right", (i + 1) * 251);
      if (rightSvg) connector.appendChild(rightSvg);

      main.appendChild(connector);
    }
  }

  // Run after all layout has settled
  function init() {
    renderStrings();

    // Recompute on resize (debounced)
    let resizeTimer;
    window.addEventListener("resize", () => {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(renderStrings, 200);
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
      requestAnimationFrame(() => requestAnimationFrame(init));
    });
  } else {
    requestAnimationFrame(() => requestAnimationFrame(init));
  }
})();
