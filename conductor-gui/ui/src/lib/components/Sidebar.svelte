<script>
  import { currentSection, navigateTo, SECTIONS } from '../stores/navigation.js';

  const navItems = [
    { id: SECTIONS.DEVICES, label: 'Devices', icon: '🎹' },
    { id: SECTIONS.MODES, label: 'Modes', icon: '🎨' },
    { id: SECTIONS.MAPPINGS, label: 'Mappings', icon: '🔗' },
    { id: SECTIONS.PLUGINS, label: 'Plugins', icon: '🧩' },
    { id: SECTIONS.SETTINGS, label: 'Settings', icon: '⚙️' },
  ];

  function handleNavClick(sectionId) {
    navigateTo(sectionId);
  }
</script>

<aside class="sidebar">
  <div class="logo">
    <h1>Conductor</h1>
    <p class="version">v2.0.0</p>
  </div>

  <nav class="nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={$currentSection === item.id}
        on:click={() => handleNavClick(item.id)}
      >
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </button>
    {/each}
  </nav>

  <div class="footer">
    <p class="copyright">© 2025 Amiable</p>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 240px;
    height: 100vh;
    background: #1a1a1a;
    border-right: 1px solid #333;
    overflow-y: auto;
  }

  .logo {
    padding: 2rem 1.5rem 1.5rem;
    border-bottom: 1px solid #333;
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #4a9eff;
    font-weight: 600;
  }

  .version {
    margin: 0.25rem 0 0;
    font-size: 0.75rem;
    color: #666;
  }

  .nav {
    flex: 1;
    padding: 1rem 0.75rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.875rem 1rem;
    margin-bottom: 0.5rem;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: #999;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;
  }

  .nav-item:hover {
    background: #252525;
    color: #e0e0e0;
  }

  .nav-item.active {
    background: #2a4a6a;
    color: #fff;
  }

  .nav-item.active:hover {
    background: #2f5274;
  }

  .icon {
    display: inline-block;
    width: 1.5rem;
    font-size: 1.25rem;
    margin-right: 0.75rem;
  }

  .label {
    font-weight: 500;
  }

  .footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid #333;
  }

  .copyright {
    margin: 0;
    font-size: 0.75rem;
    color: #666;
    text-align: center;
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .sidebar {
      width: 180px;
    }

    .logo {
      padding: 1.5rem 1rem 1rem;
    }

    .logo h1 {
      font-size: 1.25rem;
    }

    .nav-item {
      padding: 0.75rem 0.875rem;
    }

    .icon {
      font-size: 1.1rem;
    }

    .label {
      font-size: 0.875rem;
    }
  }
</style>
