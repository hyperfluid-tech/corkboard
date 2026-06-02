document.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.getElementById('sidebar');
  const overlay = document.getElementById('overlay');
  const toggleBtn = document.getElementById('sidebar-toggle');
  const articles = document.querySelectorAll('article');
  const sidebarLinks = document.querySelectorAll('.sidebar-link');

  if (sidebar) {
    sidebar.setAttribute('aria-hidden', 'true');
  }

  function toggleSidebar() {
    if (sidebar && overlay) {
      sidebar.classList.toggle('open');
      overlay.classList.toggle('open');

      const isOpen = sidebar.classList.contains('open');

      if (toggleBtn) {
        toggleBtn.setAttribute('aria-expanded', String(isOpen));
        toggleBtn.setAttribute('aria-label', isOpen ? 'Close navigation' : 'Open navigation');
      }

      sidebar.setAttribute('aria-hidden', String(!isOpen));

      const mainContainer = document.getElementById('main-container');
      const headerContainer = document.getElementById('header-container');
      const skipLink = document.querySelector('.skip-link');
      if (isOpen) {
        mainContainer?.setAttribute('inert', '');
        headerContainer?.setAttribute('inert', '');
        skipLink?.setAttribute('inert', '');
      } else {
        mainContainer?.removeAttribute('inert');
        headerContainer?.removeAttribute('inert');
        skipLink?.removeAttribute('inert');
      }
    }
  }

  if (toggleBtn) {
    toggleBtn.addEventListener('click', toggleSidebar);
  }

  if (overlay) {
    overlay.addEventListener('click', toggleSidebar);
  }

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && sidebar && sidebar.classList.contains('open')) {
      toggleSidebar();
      if (toggleBtn) {
        toggleBtn.focus();
      }
    }
  });

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

      if (isCurrent) {
        link.setAttribute('aria-current', 'true');
      } else {
        link.removeAttribute('aria-current');
      }
    });

    const activeLink = document.querySelector('.sidebar-link[aria-current="true"]');
    const indicator = document.getElementById('sidebar-active-indicator');
    const wrapper = document.querySelector('.sidebar-relative-wrapper');
    if (activeLink && indicator && wrapper) {
      const activeRect = activeLink.getBoundingClientRect();
      const wrapperRect = wrapper.getBoundingClientRect();
      const relativeTop = activeRect.top - wrapperRect.top + (activeRect.height / 2);
      
      indicator.style.transform = `translateY(${relativeTop}px) translateY(-50%) rotate(-15deg)`;
      indicator.style.opacity = '1';
    } else if (indicator) {
      indicator.style.opacity = '0';
    }
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

  window.addEventListener('resize', updateActiveLink);

  updateActiveLink();
});
