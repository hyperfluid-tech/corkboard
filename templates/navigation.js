document.addEventListener('DOMContentLoaded', () => {
  const isArticlePage = document.body.dataset.pageType === 'article';

  if (isArticlePage) {
    generateTableOfContents();
  }

  const sidebar = document.getElementById('sidebar');
  const overlay = document.getElementById('overlay');
  const toggleBtn = document.getElementById('sidebar-toggle');

  const spyTargets = isArticlePage
    ? document.querySelectorAll('.prose h1, .prose h2, .prose h3, .prose h4, .prose h5, .prose h6')
    : document.querySelectorAll('article');

  const sidebarLinks = document.querySelectorAll('.sidebar-link');

  let activeSlug = '';
  let isClickNavigating = false;
  let clickTimeout = null;

  const hash = window.location.hash.substring(1);
  if (hash && document.getElementById(hash)) {
    activeSlug = hash;
    isClickNavigating = true;
    setTimeout(() => {
      isClickNavigating = false;
      checkActiveArticle();
    }, 1500);
  }

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

  if (spyTargets.length === 0 || sidebarLinks.length === 0) return;

  let scrollTimeout = false;
  window.addEventListener('scroll', () => {
    if (isClickNavigating || spyTargets.length === 0) return;

    if (!scrollTimeout) {
      window.requestAnimationFrame(() => {
        checkActiveArticle();
        scrollTimeout = false;
      });
      scrollTimeout = true;
    }
  }, { passive: true });

  function getActiveId() {
    if (spyTargets.length === 0) return null;

    const scrollPosition = window.scrollY;
    if (scrollPosition < 50) return spyTargets[0].id;

    const maxScroll = document.documentElement.scrollHeight - window.innerHeight;
    if (scrollPosition >= maxScroll - 10 && spyTargets.length > 1) {
      if (isArticlePage) {
        return spyTargets[spyTargets.length - 1].id;
      } else {
        const secondToLast = spyTargets[spyTargets.length - 2];
        const isSecondToLastFading = secondToLast.getBoundingClientRect().bottom < window.innerHeight * 0.7;
        return isSecondToLastFading ? spyTargets[spyTargets.length - 1].id : secondToLast.id;
      }
    }

    const threshold = window.innerHeight * 0.4;
    let activeId = spyTargets[0].id;

    for (const target of spyTargets) {
      if (target.getBoundingClientRect().top <= threshold) {
        activeId = target.id;
      }
    }

    return activeId;
  }

  function checkActiveArticle() {
    const currentId = getActiveId();
    if (!currentId || currentId === activeSlug) return;

    activeSlug = currentId;
    updateActiveLink();
  }

  if (!isClickNavigating) {
    checkActiveArticle();
  } else {
    updateActiveLink();
  }

  function updateActiveLink() {
    sidebarLinks.forEach(link => {
      const isCurrent = link.getAttribute('data-slug') === activeSlug;
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

  // TOC Generator function
  function generateTableOfContents() {
    const sidebarList = document.querySelector('.sidebar-list');
    if (!sidebarList) return;

    sidebarList.innerHTML = '';

    const articleBody = document.querySelector('.prose');
    if (!articleBody) return;

    const headings = articleBody.querySelectorAll('h1, h2, h3, h4, h5, h6');
    if (headings.length === 0) {
      const li = document.createElement('li');
      li.innerHTML = '<span class="block p-2 text-outline italic">No headings</span>';
      sidebarList.appendChild(li);
      return;
    }

    headings.forEach((heading, index) => {
      if (!heading.id) {
        const text = heading.textContent.trim();
        const slug = text.toLowerCase()
          .replace(/[^a-z0-9]+/g, '-')
          .replace(/(^-|-$)/g, '');
        heading.id = slug || `heading-${index + 1}`;
      }

      const level = parseInt(heading.tagName.substring(1));

      const li = document.createElement('li');
      li.className = 'sidebar-item';

      const a = document.createElement('a');
      a.className = 'block hover:bg-white/30 p-2 rounded transition-colors sidebar-link font-semibold overflow-hidden';
      a.href = `#${heading.id}`;
      a.setAttribute('data-slug', heading.id);

      // Indent: H1 = 0px, H2 = 12px, H3 = 24px, etc.
      const indent = (level - 1) * 12;
      a.style.setProperty('--toc-indent', `${indent}px`);

      // Style hierarchy:
      if (level >= 5) {
        a.classList.add('text-xs', 'font-normal', 'text-primary/75', 'italic');
        a.classList.remove('font-semibold');
      } else if (level >= 3) {
        a.classList.add('text-sm', 'font-medium', 'text-primary/90');
        a.classList.remove('font-semibold');
      } else {
        a.classList.add('font-semibold', 'text-primary');
      }

      const span = document.createElement('span');
      span.className = 'sidebar-link-text block lg:truncate';
      span.textContent = heading.textContent.trim();

      a.appendChild(span);
      li.appendChild(a);
      sidebarList.appendChild(li);
    });
  }
});
