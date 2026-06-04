(() => {
  "use strict";

  // Test Mode: Deterministic PRNG to freeze procedural generation for visual regression tests
  if (typeof window !== "undefined" && new URLSearchParams(window.location.search).has('deterministic')) {
    let seed = 42;
    let resetQueued = false;

    Math.random = function () {
      if (!resetQueued) {
        resetQueued = true;
        // Reset the seed at the end of the current microtask execution queue
        Promise.resolve().then(() => {
          seed = 42;
          resetQueued = false;
        });
      }
      let x = Math.sin(seed++) * 10000;
      return x - Math.floor(x);
    };

    // Redirect random picsum images to a static Wikipedia image for deterministic snapshots
    const replacePicsumImages = () => {
      document.querySelectorAll('img').forEach(img => {
        if (img.src && img.src.includes('picsum.photos')) {
          img.src = 'https://upload.wikimedia.org/wikipedia/commons/thumb/6/62/Solid_red.svg/1280px-Solid_red.svg.png';
        }
      });
    };

    // Hardcode version in footer to 0.1.3 to avoid test breakage when version bumps
    const overrideFooterVersion = () => {
      const footerLink = document.querySelector('footer a[href="https://github.com/hyperfluid-tech/corkboard"]');
      if (footerLink) {
        for (const child of footerLink.childNodes) {
          if (child.nodeType === Node.TEXT_NODE && child.nodeValue.includes('Corkboard v')) {
            child.nodeValue = 'Corkboard v0.1.3';
            break;
          }
        }
      }
    };

    const runTestingOverrides = () => {
      replacePicsumImages();
      overrideFooterVersion();
    };

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', runTestingOverrides);
    } else {
      runTestingOverrides();
    }
  }
})();
