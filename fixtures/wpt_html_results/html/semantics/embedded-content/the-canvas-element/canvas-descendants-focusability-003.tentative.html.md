# html/semantics/embedded-content/the-canvas-element/canvas-descendants-focusability-003.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-canvas-element/canvas-descendants-focusability-003.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Canvas descendants focusability</title>
<link rel="author" title="Oriol Brufau" href="mailto:obrufau@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#being-used-as-relevant-canvas-fallback-content">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#focusable-area">
<link rel="help" href="https://github.com/whatwg/html/issues/7534">
<meta name="assert" content="Checks that elements being used as relevant canvas
    fallback content can't be focusable if they are not rendered because of an
    explicit 'display: none' style, but can if they are not rendered because of
    a 'display: contents' style.">
<div id="log"></div>
<canvas>
  <button hidden data-focusable="false"></button>
  <section hidden tabindex="-1" data-focusable="false">
    <div tabindex="-1" data-focusable="false"></div>
    <span tabindex="-1" data-focusable="false"></span>
    <a href="#" data-focusable="false"></a>
  </section>
  <button style="display: contents" data-focusable="true"></button>
  <section style="display: contents" tabindex="-1" data-focusable="true">
    <div style="display: contents" tabindex="-1" data-focusable="true"></div>
    <span style="display: contents" tabindex="-1" data-focusable="true"></span>
    <a style="display: contents" href="#" data-focusable="true"></a>
  </section>
</canvas>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup(() => {
  const canvas = document.querySelector("canvas");
  assert_greater_than(canvas.getClientRects().length, 0, "Canvas is rendered");
});
for (let element of document.querySelectorAll("[data-focusable]")) {
  let title = element.cloneNode(false).outerHTML.toLowerCase();
  title = title.slice(0, title.lastIndexOf("<"));
  test(function() {
    assert_equals(element.getClientRects().length, 0, "Not rendered");
    assert_true(document.activeElement !== element, "Not initially focused");
    element.focus();
    if (JSON.parse(element.dataset.focusable)) {
      assert_true(document.activeElement === element, "Should be focused");
    } else {
      assert_true(document.activeElement !== element, "Shouldn't be focused");
    }
  }, title);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 1023,
        "byte_start": 1013,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 1405,
        "byte_start": 1395,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-canvas-element/canvas-descendants-focusability-003.tentative.html"
}
```
