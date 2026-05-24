document.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.getElementById('sidebar');
  const overlay = document.getElementById('overlay');
  const toggleBtn = document.getElementById('sidebar-toggle');
  const articles = document.querySelectorAll('article');
  const sidebarLinks = document.querySelectorAll('.sidebar-link');

  function toggleSidebar() {
    if (sidebar && overlay) {
      sidebar.classList.toggle('open');
      overlay.classList.toggle('open');
    }
  }

  if (toggleBtn) {
    toggleBtn.addEventListener('click', toggleSidebar);
  }

  if (overlay) {
    overlay.addEventListener('click', toggleSidebar);
  }

  sidebarLinks.forEach(link => {
    link.addEventListener('click', () => {
      if (window.innerWidth < 1024) {
        toggleSidebar();
      }
    });

    const textSpan = link.querySelector('.sidebar-link-text');
    if (textSpan) {
      link.addEventListener('mouseenter', () => {
        if (window.innerWidth < 1024) return;

        const textWidth = textSpan.scrollWidth;
        const containerWidth = link.clientWidth - 16;
        if (textWidth > containerWidth) {
          const scrollDist = textWidth - containerWidth + 4;
          const duration = scrollDist / 40;
          link.style.setProperty('--scroll-dist', `-${scrollDist}px`);
          link.style.setProperty('--scroll-duration', `${duration}s`);
        }
      });

      link.addEventListener('mouseleave', () => {
        link.style.removeProperty('--scroll-dist');
        link.style.removeProperty('--scroll-duration');
      });
    }
  });

  if (articles.length === 0 || sidebarLinks.length === 0) return;

  const observerOptions = {
    root: null,
    rootMargin: '-20% 0px -60% 0px',
    threshold: 0
  };

  let activeSlug = '';

  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        activeSlug = entry.target.id;
        updateActiveLink();
      }
    });
  }, observerOptions);

  articles.forEach(article => observer.observe(article));

  function updateActiveLink() {
    let currentActive = activeSlug;
    const scrollPosition = window.scrollY;
    const maxScroll = document.documentElement.scrollHeight - window.innerHeight;

    if (scrollPosition < 50) {
      currentActive = articles[0].id;
    } else if (scrollPosition >= maxScroll - 50) {
      currentActive = articles[articles.length - 1].id;
    }

    sidebarLinks.forEach(link => {
      const isCurrent = link.getAttribute('data-slug') === currentActive;
      link.classList.toggle('font-bold', isCurrent);
      link.classList.toggle('font-semibold', !isCurrent);
    });
  }

  let ticking = false;
  window.addEventListener('scroll', () => {
    if (!ticking) {
      window.requestAnimationFrame(() => {
        updateActiveLink();
        ticking = false;
      });
      ticking = true;
    }
  });

  updateActiveLink();
});
