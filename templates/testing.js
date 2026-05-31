(() => {
  "use strict";

  // Test Mode: Deterministic PRNG to freeze procedural generation for visual regression tests
  if (typeof window !== "undefined" && new URLSearchParams(window.location.search).has('deterministic')) {
    let seed = 42;
    let resetQueued = false;
    
    Math.random = function() {
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
          img.src = 'https://upload.wikimedia.org/wikipedia/commons/thumb/9/9f/Pride_of_Madeira.%28Echium_candicans%29_%2814327908576%29.jpg/1280px-Pride_of_Madeira.%28Echium_candicans%29_%2814327908576%29.jpg';
        }
      });
    };

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', replacePicsumImages);
    } else {
      replacePicsumImages();
    }
  }
})();
