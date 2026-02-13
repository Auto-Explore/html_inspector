# html/semantics/popovers/popover-anchor-scroll-display.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-scroll-display.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<meta charset="utf-8">
<link rel=author href="mailto:xiaochengh@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=match href="popover-anchor-scroll-display-ref.html">

<div class=spacer style="height: 200px"></div>

<p>There should be a green box attached to the right side of each orange box.</p>

<!-- Example using the `anchor` implicit reference element -->
<div class=ex>
  <div class=anchor id=anchor1></div>
  <div id=popover1 popover=manual anchor=anchor1></div>
</div>

<!-- Example using a default anchor that is not the implicit anchor -->
<div class=ex>
  <div class=anchor id=anchor2></div>
  <div id=popover2 popover=manual anchor=fake-anchor></div>
</div>

<!-- A position:fixed fake anchor. Any popover anchored to it won't move when
     the document is scrolled. -->
<div id=fake-anchor></div>

<div class=spacer style="height: 200vh"></div>

<style>
  .ex {
    margin: 25px;
  }
  .ex div {
    width: 100px;
    height: 100px;
  }
  .anchor {
    background: orange;
  }
  [popover] {
    inset: auto;
    background: lime;
    padding:0;
    border:0;
  }
  #popover1 {
    left: anchor(right);
    top: anchor(top);
  }
  #fake-anchor {
    position: fixed;
    anchor-name: --fake-anchor;
  }
  #anchor2 {
    anchor-name: --anchor2;
  }
  #popover2 {
    position-anchor: --anchor2;
    left: anchor(right);
    top: anchor(top);
  }
</style>

<script>
function raf() {
  return new Promise(resolve => requestAnimationFrame(resolve));
}

async function runTest() {
  document.querySelectorAll('[popover]').forEach(
      popover => popover.showPopover());

  // Render a frame at the intial scroll position.
  await raf();
  await raf();

  document.documentElement.scrollTop = 100;
  document.documentElement.classList.remove('reftest-wait');

  // The popover should still be attached to the anchor.
}
runTest();
</script>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1043,
        "byte_start": 1036,
        "col": 1,
        "line": 31
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
  "source_name": "html/semantics/popovers/popover-anchor-scroll-display.tentative.html"
}
```
