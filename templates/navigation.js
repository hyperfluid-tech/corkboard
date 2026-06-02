document.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.getElementById('sidebar');
  const overlay = document.getElementById('overlay');
  const toggleBtn = document.getElementById('sidebar-toggle');
  const articles = document.querySelectorAll('article');
  const sidebarLinks = document.querySelectorAll('.sidebar-link');
  
  let activeSlug = '';
  let isClickNavigating = false;
  let clickTimeout = null;

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
      isClickNavigating = true;
      activeSlug = link.getAttribute('data-slug');
      updateActiveLink();

      clearTimeout(clickTimeout);
      clickTimeout = setTimeout(() => {
        isClickNavigating = false;
      }, 800);

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

  const sidebarItems = document.querySelectorAll('.sidebar-item');
  sidebarItems.forEach(item => {
    const link = item.querySelector('.sidebar-link');
    if (!link) return;

    item.addEventListener('mouseenter', () => {
      const indicator = document.getElementById('sidebar-active-indicator');
      if (indicator && indicator.style.opacity !== '0') {
        const wrapper = document.querySelector('.sidebar-relative-wrapper');

        if (link.getAttribute('aria-current') === 'true') {
          indicator.style.setProperty('--indicator-angle', `-15deg`);
          if (wrapper) {
            const rect = link.getBoundingClientRect();
            const wrapperRect = wrapper.getBoundingClientRect();
            indicator.style.setProperty('--indicator-top', `${rect.top - wrapperRect.top + rect.height / 2}px`);
          }
        } else {
          const linksArray = Array.from(sidebarLinks);
          const hoveredIndex = linksArray.indexOf(link);
          const activeIndex = linksArray.findIndex(l => l.getAttribute('aria-current') === 'true');

          if (activeIndex !== -1) {
            const distance = hoveredIndex - activeIndex;
            let angle = -15 + (distance * 15);

            if (angle > 45) angle = 45;
            if (angle < -45) angle = -45;

            indicator.style.setProperty('--indicator-angle', `${angle}deg`);

            if (wrapper) {
              const rect = link.getBoundingClientRect();
              const wrapperRect = wrapper.getBoundingClientRect();
              indicator.style.setProperty('--indicator-top', `${rect.top - wrapperRect.top + rect.height / 2}px`);
            }
          }
        }
      }
    });

    item.addEventListener('mouseleave', () => {
      const indicator = document.getElementById('sidebar-active-indicator');
      const activeLink = document.querySelector('.sidebar-link[aria-current="true"]');
      const wrapper = document.querySelector('.sidebar-relative-wrapper');

      if (indicator && activeLink && wrapper) {
        indicator.style.setProperty('--indicator-angle', `-15deg`);
        const activeRect = activeLink.getBoundingClientRect();
        const wrapperRect = wrapper.getBoundingClientRect();
        indicator.style.setProperty('--indicator-top', `${activeRect.top - wrapperRect.top + activeRect.height / 2}px`);
      }
    });
  });

  if (articles.length === 0 || sidebarLinks.length === 0) return;

  const observerOptions = {
    root: null,
    rootMargin: '-10% 0px -40% 0px',
    threshold: 0
  };

  const intersectingArticles = new Set();

  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        intersectingArticles.add(entry.target);
      } else {
        intersectingArticles.delete(entry.target);
      }
    });

    if (isClickNavigating) return;

    if (intersectingArticles.size > 0) {
      const active = Array.from(intersectingArticles).sort((a, b) => {
        return a.getBoundingClientRect().top - b.getBoundingClientRect().top;
      })[0];
      
      if (active.id !== activeSlug) {
        activeSlug = active.id;
        updateActiveLink();
      }
    }
  }, observerOptions);

  articles.forEach(article => observer.observe(article));

  function updateActiveLink() {
    let currentActive = activeSlug;
    const scrollPosition = window.scrollY;

    if (scrollPosition < 50) {
      currentActive = articles[0].id;
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

      indicator.style.setProperty('--indicator-top', `${relativeTop}px`);
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
