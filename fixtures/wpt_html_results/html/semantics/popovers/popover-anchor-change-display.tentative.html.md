# html/semantics/popovers/popover-anchor-change-display.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-change-display.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:xiaochengh@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=match href="popover-anchor-change-display-ref.html">
<script src="resources/popover-utils.js"></script>

<p>There should be a green box attached to the right side of each orange box.</p>

<div class=ex>
  <div class=anchor id=anchor1></div>
  <div id=popover1 popover=manual defaultopen></div>
</div>

<div class=ex>
  <div class=anchor id=will-be-anchor2></div>
  <div id=popover2 popover=manual anchor=anchor2 defaultopen></div>
</div>

<script>
showDefaultopenPopoversOnLoad();

function runTest() {
  document.body.offsetLeft; // Force layout

  document.getElementById('popover1').setAttribute('anchor', 'anchor1');
  document.getElementById('will-be-anchor2').setAttribute('id', 'anchor2');
}
window.addEventListener('load', runTest);
</script>

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
    left: anchor(right);
    top: anchor(top);
  }
</style>
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
        "byte_end": 1020,
        "byte_start": 1013,
        "col": 1,
        "line": 33
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
  "source_name": "html/semantics/popovers/popover-anchor-change-display.tentative.html"
}
```
