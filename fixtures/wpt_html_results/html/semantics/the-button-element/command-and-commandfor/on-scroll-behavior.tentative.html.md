# html/semantics/the-button-element/command-and-commandfor/on-scroll-behavior.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-scroll-behavior.tentative.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Chromium" href="https://chromium.org" />
<meta name="timeout" content="long" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<style>
  .scroll-container {
    width: 200px;
    height: 200px;
    overflow: auto;
    border: 1px solid black;
  }

  .scroll-content {
    width: 1000px;
    height: 1000px;
    background: linear-gradient(to bottom right, red, blue);
  }

  .scroll-container-horizontal {
    width: 200px;
    height: 100px;
    overflow-x: auto;
    overflow-y: hidden;
    border: 1px solid black;
  }

  .scroll-content-horizontal {
    width: 1000px;
    height: 100px;
    background: linear-gradient(to right, red, blue);
  }

  .scroll-container-vertical {
    width: 100px;
    height: 200px;
    overflow-y: auto;
    overflow-x: hidden;
    border: 1px solid black;
  }

  .scroll-content-vertical {
    width: 100px;
    height: 1000px;
    background: linear-gradient(to bottom, red, blue);
  }

  .rtl {
    direction: rtl;
  }

  .vertical-writing {
    writing-mode: vertical-rl;
  }
</style>

<!-- Basic scroll container -->
<div id="scrollcontainer" class="scroll-container">
  <div class="scroll-content"></div>
</div>
<button id="pageup" commandfor="scrollcontainer" command="page-up">Page Up</button>
<button id="pagedown" commandfor="scrollcontainer" command="page-down">Page Down</button>
<button id="pageleft" commandfor="scrollcontainer" command="page-left">Page Left</button>
<button id="pageright" commandfor="scrollcontainer" command="page-right">Page Right</button>

<!-- Horizontal only scroll container -->
<div id="horizontalcontainer" class="scroll-container-horizontal">
  <div class="scroll-content-horizontal"></div>
</div>
<button id="hpageleft" commandfor="horizontalcontainer" command="page-left">Page Left</button>
<button id="hpageright" commandfor="horizontalcontainer" command="page-right">Page Right</button>

<!-- Vertical only scroll container -->
<div id="verticalcontainer" class="scroll-container-vertical">
  <div class="scroll-content-vertical"></div>
</div>
<button id="vpageup" commandfor="verticalcontainer" command="page-up">Page Up</button>
<button id="vpagedown" commandfor="verticalcontainer" command="page-down">Page Down</button>

<!-- Logical direction tests -->
<div id="logicalcontainer" class="scroll-container">
  <div class="scroll-content"></div>
</div>
<button id="blockstart" commandfor="logicalcontainer" command="page-block-start">Block Start</button>
<button id="blockend" commandfor="logicalcontainer" command="page-block-end">Block End</button>
<button id="inlinestart" commandfor="logicalcontainer" command="page-inline-start">Inline Start</button>
<button id="inlineend" commandfor="logicalcontainer" command="page-inline-end">Inline End</button>

<!-- RTL container -->
<div id="rtlcontainer" class="scroll-container rtl">
  <div class="scroll-content"></div>
</div>
<button id="rtlinlinestart" commandfor="rtlcontainer" command="page-inline-start">Inline Start (RTL)</button>
<button id="rtlinlineend" commandfor="rtlcontainer" command="page-inline-end">Inline End (RTL)</button>

<!-- Vertical writing mode container -->
<div id="verticalwritingcontainer" class="scroll-container vertical-writing">
  <div class="scroll-content"></div>
</div>
<button id="vwblockstart" commandfor="verticalwritingcontainer" command="page-block-start">Block Start (VW)</button>
<button id="vwblockend" commandfor="verticalwritingcontainer" command="page-block-end">Block End (VW)</button>

<script>
  function resetScrollPosition(container) {
    container.scrollTop = 0;
    container.scrollLeft = 0;
  }

  // Test page-up command
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    scrollcontainer.scrollTop = 400;
    const initialScrollTop = scrollcontainer.scrollTop;
    pageup.click();
    assert_less_than(scrollcontainer.scrollTop, initialScrollTop, "Scroll position should decrease");
  }, "page-up command scrolls up");

  // Test page-down command
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    const initialScrollTop = scrollcontainer.scrollTop;
    pagedown.click();
    assert_greater_than(scrollcontainer.scrollTop, initialScrollTop, "Scroll position should increase");
  }, "page-down command scrolls down");

  // Test page-left command
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    scrollcontainer.scrollLeft = 400;
    const initialScrollLeft = scrollcontainer.scrollLeft;
    pageleft.click();
    assert_less_than(scrollcontainer.scrollLeft, initialScrollLeft, "Scroll position should decrease");
  }, "page-left command scrolls left");

  // Test page-right command
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    const initialScrollLeft = scrollcontainer.scrollLeft;
    pageright.click();
    assert_greater_than(scrollcontainer.scrollLeft, initialScrollLeft, "Scroll position should increase");
  }, "page-right command scrolls right");

  // Test that page-up doesn't scroll horizontally
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    scrollcontainer.scrollTop = 400;
    scrollcontainer.scrollLeft = 200;
    const initialScrollLeft = scrollcontainer.scrollLeft;
    pageup.click();
    assert_equals(scrollcontainer.scrollLeft, initialScrollLeft, "Horizontal scroll should not change");
  }, "page-up command doesn't affect horizontal scroll");

  // Test that page-left doesn't scroll vertically
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    scrollcontainer.scrollTop = 200;
    scrollcontainer.scrollLeft = 400;
    const initialScrollTop = scrollcontainer.scrollTop;
    pageleft.click();
    assert_equals(scrollcontainer.scrollTop, initialScrollTop, "Vertical scroll should not change");
  }, "page-left command doesn't affect vertical scroll");

  // Test horizontal-only container
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(horizontalcontainer));
    const initialScrollLeft = horizontalcontainer.scrollLeft;
    hpageright.click();
    assert_greater_than(horizontalcontainer.scrollLeft, initialScrollLeft, "Horizontal scroll should increase");
    assert_equals(horizontalcontainer.scrollTop, 0, "Vertical scroll should remain 0");
  }, "page-right works on horizontal-only container");

  // Test vertical-only container
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(verticalcontainer));
    const initialScrollTop = verticalcontainer.scrollTop;
    vpagedown.click();
    assert_greater_than(verticalcontainer.scrollTop, initialScrollTop, "Vertical scroll should increase");
    assert_equals(verticalcontainer.scrollLeft, 0, "Horizontal scroll should remain 0");
  }, "page-down works on vertical-only container");

  // Test page-block-end (should scroll down in horizontal writing mode)
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(logicalcontainer));
    const initialScrollTop = logicalcontainer.scrollTop;
    blockend.click();
    assert_greater_than(logicalcontainer.scrollTop, initialScrollTop, "Scroll position should increase");
  }, "page-block-end scrolls down in horizontal writing mode");

  // Test page-block-start (should scroll up in horizontal writing mode)
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(logicalcontainer));
    logicalcontainer.scrollTop = 400;
    const initialScrollTop = logicalcontainer.scrollTop;
    blockstart.click();
    assert_less_than(logicalcontainer.scrollTop, initialScrollTop, "Scroll position should decrease");
  }, "page-block-start scrolls up in horizontal writing mode");

  // Test page-inline-end (should scroll right in LTR)
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(logicalcontainer));
    const initialScrollLeft = logicalcontainer.scrollLeft;
    inlineend.click();
    assert_greater_than(logicalcontainer.scrollLeft, initialScrollLeft, "Scroll position should increase");
  }, "page-inline-end scrolls right in LTR");

  // Test page-inline-start (should scroll left in LTR)
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(logicalcontainer));
    logicalcontainer.scrollLeft = 400;
    const initialScrollLeft = logicalcontainer.scrollLeft;
    inlinestart.click();
    assert_less_than(logicalcontainer.scrollLeft, initialScrollLeft, "Scroll position should decrease");
  }, "page-inline-start scrolls left in LTR");

  // Test RTL inline directions
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(rtlcontainer));
    // In RTL, inline-end should scroll left (in the visual sense)
    const initialScrollLeft = rtlcontainer.scrollLeft;
    rtlinlineend.click();
    // Note: RTL scrolling behavior can vary, but the command should work
    assert_not_equals(rtlcontainer.scrollLeft, initialScrollLeft, "Scroll position should change");
  }, "page-inline-end works in RTL container");

  // Test case insensitivity
  ["page-up", "PAGE-UP", "PaGe-Up"].forEach((command) => {
    test(function (t) {
      t.add_cleanup(() => resetScrollPosition(scrollcontainer));
      const button = document.createElement("button");
      button.setAttribute("commandfor", "scrollcontainer");
      button.setAttribute("command", command);
      document.body.appendChild(button);
      t.add_cleanup(() => button.remove());

      scrollcontainer.scrollTop = 400;
      const initialScrollTop = scrollcontainer.scrollTop;
      button.click();
      assert_less_than(scrollcontainer.scrollTop, initialScrollTop, "Scroll should work with " + command);
    }, `scroll command is case-insensitive: ${command}`);
  });

  // Test preventDefault
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    scrollcontainer.addEventListener("command", (e) => e.preventDefault(), { once: true });
    const initialScrollTop = scrollcontainer.scrollTop;
    pagedown.click();
    assert_equals(scrollcontainer.scrollTop, initialScrollTop, "Scroll should not change when prevented");
  }, "preventDefault stops scroll command");

  // Test that scroll doesn't happen if commandfor is invalid
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    const button = document.createElement("button");
    button.setAttribute("commandfor", "nonexistent");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    const initialScrollTop = scrollcontainer.scrollTop;
    button.click();
    assert_equals(scrollcontainer.scrollTop, initialScrollTop, "Scroll should not happen with invalid commandfor");
  }, "scroll command requires valid commandfor target");

  // Test that scroll doesn't happen on non-scrollable element
  test(function (t) {
    const nonscrollable = document.createElement("div");
    nonscrollable.id = "nonscrollable";
    nonscrollable.textContent = "Not scrollable";
    document.body.appendChild(nonscrollable);
    t.add_cleanup(() => nonscrollable.remove());

    const button = document.createElement("button");
    button.setAttribute("commandfor", "nonscrollable");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw or cause issues
    button.click();
    assert_equals(nonscrollable.scrollTop, 0, "Non-scrollable element should remain at 0");
  }, "scroll command on non-scrollable element does nothing");

  // Test scroll amount is reasonable (approximately one page)
  test(function (t) {
    t.add_cleanup(() => resetScrollPosition(scrollcontainer));
    const initialScrollTop = scrollcontainer.scrollTop;
    const containerHeight = scrollcontainer.clientHeight;
    pagedown.click();
    const scrollDistance = scrollcontainer.scrollTop - initialScrollTop;

    // Scroll should be at least 80% of container height (allowing for some overlap)
    assert_greater_than(scrollDistance, containerHeight * 0.8,
      "Scroll distance should be approximately one page");
    // And not more than 1.2x container height
    assert_less_than(scrollDistance, containerHeight * 1.2,
      "Scroll distance should not be much more than one page");
  }, "scroll amount is approximately one page");

  // Edge case: commandfor references non-existent element
  test(function (t) {
    const button = document.createElement("button");
    button.setAttribute("commandfor", "this-element-does-not-exist");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw");
  }, "scroll command with non-existent commandfor target doesn't throw");

  // Edge case: commandfor is empty string
  test(function (t) {
    const button = document.createElement("button");
    button.setAttribute("commandfor", "");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw");
  }, "scroll command with empty commandfor doesn't throw");

  // Edge case: commandfor is whitespace
  test(function (t) {
    const button = document.createElement("button");
    button.setAttribute("commandfor", "   ");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw");
  }, "scroll command with whitespace commandfor doesn't throw");

  // Edge case: target element is disconnected
  test(function (t) {
    const disconnected = document.createElement("div");
    disconnected.id = "disconnected";
    disconnected.className = "scroll-container";
    disconnected.innerHTML = '<div class="scroll-content"></div>';

    const button = document.createElement("button");
    button.setAttribute("commandfor", "disconnected");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw for disconnected target");
  }, "scroll command with disconnected target element doesn't throw");

  // Edge case: target element is button itself
  test(function (t) {
    const button = document.createElement("button");
    button.id = "selfbutton";
    button.setAttribute("commandfor", "selfbutton");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw when targeting self");
  }, "scroll command targeting self doesn't throw");

  // Edge case: target element is display:none
  test(function (t) {
    const hidden = document.createElement("div");
    hidden.id = "hiddenscroll";
    hidden.className = "scroll-container";
    hidden.style.display = "none";
    hidden.innerHTML = '<div class="scroll-content"></div>';
    document.body.appendChild(hidden);
    t.add_cleanup(() => hidden.remove());

    const button = document.createElement("button");
    button.setAttribute("commandfor", "hiddenscroll");
    button.setAttribute("command", "page-down");
    document.body.appendChild(button);
    t.add_cleanup(() => button.remove());

    // Should not throw
    assert_equals(button.click(), undefined, "Click should not throw for hidden target");
    assert_equals(hidden.scrollTop, 0, "Hidden element should not scroll");
  }, "scroll command on display:none element does nothing");

  // Edge case: target element has no computed style (e.g., in detached document)
  test(function (t) {
    const newDoc = document.implementation.createHTMLDocument();
    const container = newDoc.createElement("div");
    container.id = "detachedcontainer";
    container.className = "scroll-container";
    newDoc.bo
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 108,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 108,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-scroll-behavior.tentative.html"
}
```
