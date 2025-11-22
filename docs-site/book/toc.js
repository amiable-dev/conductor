// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item affix "><li class="part-title">Installation</li><li class="chapter-item "><a href="installation/macos.html"><strong aria-hidden="true">1.</strong> macOS Installation</a></li><li class="chapter-item "><a href="installation/linux.html"><strong aria-hidden="true">2.</strong> Linux Installation</a></li><li class="chapter-item "><a href="installation/windows.html"><strong aria-hidden="true">3.</strong> Windows Installation</a></li><li class="chapter-item "><a href="installation/building.html"><strong aria-hidden="true">4.</strong> Building from Source</a></li><li class="chapter-item affix "><li class="part-title">Getting Started</li><li class="chapter-item "><a href="getting-started/quick-start.html"><strong aria-hidden="true">5.</strong> Quick Start</a></li><li class="chapter-item "><a href="getting-started/first-mapping.html"><strong aria-hidden="true">6.</strong> Your First Mapping</a></li><li class="chapter-item "><a href="getting-started/midi-learn.html"><strong aria-hidden="true">7.</strong> MIDI Learn</a></li><li class="chapter-item "><a href="getting-started/modes.html"><strong aria-hidden="true">8.</strong> Understanding Modes</a></li><li class="chapter-item affix "><li class="part-title">Guides</li><li class="chapter-item "><a href="guides/daemon.html"><strong aria-hidden="true">9.</strong> Daemon &amp; Hot-Reload</a></li><li class="chapter-item "><a href="guides/gui.html"><strong aria-hidden="true">10.</strong> GUI Configuration</a></li><li class="chapter-item "><a href="guides/device-templates.html"><strong aria-hidden="true">11.</strong> Device Templates</a></li><li class="chapter-item "><a href="guides/per-app-profiles.html"><strong aria-hidden="true">12.</strong> Per-App Profiles</a></li><li class="chapter-item "><a href="guides/led-system.html"><strong aria-hidden="true">13.</strong> LED System</a></li><li class="chapter-item "><a href="guides/event-console.html"><strong aria-hidden="true">14.</strong> Event Console</a></li><li class="chapter-item "><a href="guides/velocity-curves.html"><strong aria-hidden="true">15.</strong> Velocity Curves</a></li><li class="chapter-item "><a href="guides/context-aware.html"><strong aria-hidden="true">16.</strong> Context-Aware Mappings</a></li><li class="chapter-item "><a href="guides/daw-control.html"><strong aria-hidden="true">17.</strong> DAW Control</a></li><li class="chapter-item affix "><li class="part-title">Tutorials</li><li class="chapter-item "><a href="tutorials/dynamic-workflows.html"><strong aria-hidden="true">18.</strong> Dynamic Workflows</a></li><li class="chapter-item affix "><li class="part-title">Examples</li><li class="chapter-item "><a href="examples/logic-pro.html"><strong aria-hidden="true">19.</strong> Logic Pro Integration</a></li><li class="chapter-item "><a href="examples/ableton-live.html"><strong aria-hidden="true">20.</strong> Ableton Live Integration</a></li><li class="chapter-item affix "><li class="part-title">Configuration</li><li class="chapter-item "><a href="configuration/overview.html"><strong aria-hidden="true">21.</strong> Configuration Overview</a></li><li class="chapter-item "><a href="configuration/triggers.html"><strong aria-hidden="true">22.</strong> Triggers Reference</a></li><li class="chapter-item "><a href="configuration/actions.html"><strong aria-hidden="true">23.</strong> Actions Reference</a></li><li class="chapter-item "><a href="configuration/curves.html"><strong aria-hidden="true">24.</strong> Velocity Mappings</a></li><li class="chapter-item "><a href="configuration/conditionals.html"><strong aria-hidden="true">25.</strong> Conditional Actions</a></li><li class="chapter-item "><a href="configuration/modes.html"><strong aria-hidden="true">26.</strong> Modes System</a></li><li class="chapter-item "><a href="configuration/led-feedback.html"><strong aria-hidden="true">27.</strong> LED Feedback</a></li><li class="chapter-item "><a href="configuration/device-profiles.html"><strong aria-hidden="true">28.</strong> Device Profiles</a></li><li class="chapter-item "><a href="configuration/examples.html"><strong aria-hidden="true">29.</strong> Configuration Examples</a></li><li class="chapter-item affix "><li class="part-title">Reference</li><li class="chapter-item "><a href="reference/trigger-types.html"><strong aria-hidden="true">30.</strong> Trigger Types</a></li><li class="chapter-item "><a href="reference/action-types.html"><strong aria-hidden="true">31.</strong> Action Types</a></li><li class="chapter-item "><a href="reference/led-system.html"><strong aria-hidden="true">32.</strong> LED System</a></li><li class="chapter-item "><a href="reference/cli-commands.html"><strong aria-hidden="true">33.</strong> CLI Commands</a></li><li class="chapter-item "><a href="reference/config-schema.html"><strong aria-hidden="true">34.</strong> Config File Schema</a></li><li class="chapter-item "><a href="reference/environment.html"><strong aria-hidden="true">35.</strong> Environment Variables</a></li><li class="chapter-item affix "><li class="part-title">Device Support</li><li class="chapter-item "><a href="devices/compatibility.html"><strong aria-hidden="true">36.</strong> Compatibility Matrix</a></li><li class="chapter-item "><a href="devices/mikro-mk3.html"><strong aria-hidden="true">37.</strong> Maschine Mikro MK3</a></li><li class="chapter-item "><a href="devices/generic-midi.html"><strong aria-hidden="true">38.</strong> Generic MIDI Controllers</a></li><li class="chapter-item "><a href="devices/creating-profiles.html"><strong aria-hidden="true">39.</strong> Creating Device Profiles</a></li><li class="chapter-item affix "><li class="part-title">Development</li><li class="chapter-item "><a href="development/setup.html"><strong aria-hidden="true">40.</strong> Development Setup</a></li><li class="chapter-item "><a href="development/architecture.html"><strong aria-hidden="true">41.</strong> Architecture Overview</a></li><li class="chapter-item "><a href="development/plugin-development.html"><strong aria-hidden="true">42.</strong> Plugin Development</a></li><li class="chapter-item "><a href="development/wasm-plugins.html"><strong aria-hidden="true">43.</strong> WASM Plugins</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="development/wasm-plugin-development.html"><strong aria-hidden="true">43.1.</strong> WASM Plugin Development</a></li><li class="chapter-item "><a href="development/plugin-security.html"><strong aria-hidden="true">43.2.</strong> Plugin Security</a></li><li class="chapter-item "><a href="development/plugin-examples.html"><strong aria-hidden="true">43.3.</strong> Plugin Examples</a></li></ol></li><li class="chapter-item "><a href="development/contributing.html"><strong aria-hidden="true">44.</strong> Contributing Guide</a></li><li class="chapter-item "><a href="development/testing.html"><strong aria-hidden="true">45.</strong> Testing Guide</a></li><li class="chapter-item "><a href="development/release-process.html"><strong aria-hidden="true">46.</strong> Release Process</a></li><li class="chapter-item affix "><li class="part-title">Troubleshooting</li><li class="chapter-item "><a href="troubleshooting/common-issues.html"><strong aria-hidden="true">47.</strong> Common Issues</a></li><li class="chapter-item "><a href="troubleshooting/midi-output.html"><strong aria-hidden="true">48.</strong> MIDI Output Issues</a></li><li class="chapter-item "><a href="troubleshooting/faq.html"><strong aria-hidden="true">49.</strong> FAQ</a></li><li class="chapter-item "><a href="troubleshooting/diagnostics.html"><strong aria-hidden="true">50.</strong> Diagnostic Tools</a></li><li class="chapter-item "><a href="troubleshooting/performance.html"><strong aria-hidden="true">51.</strong> Performance</a></li><li class="chapter-item affix "><li class="part-title">Resources</li><li class="chapter-item "><a href="resources/changelog.html"><strong aria-hidden="true">52.</strong> Changelog</a></li><li class="chapter-item "><a href="resources/roadmap.html"><strong aria-hidden="true">53.</strong> Roadmap</a></li><li class="chapter-item "><a href="resources/community.html"><strong aria-hidden="true">54.</strong> Community</a></li><li class="chapter-item "><a href="resources/support.html"><strong aria-hidden="true">55.</strong> Support</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
