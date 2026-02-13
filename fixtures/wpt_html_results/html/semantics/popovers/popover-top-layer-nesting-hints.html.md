# html/semantics/popovers/popover-top-layer-nesting-hints.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-top-layer-nesting-hints.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://open-ui.org/components/popover-hint.research.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/common/top-layer.js"></script>
<script src="resources/popover-utils.js"></script>
<script src="resources/popover-top-layer-nesting.js"></script>

<div id=tests>
  <div> Single popover=hint ancestor
    <div popover=hint class=target data-stay-open=true></div>
  </div>

  <div> Nested auto/hint ancestors
    <div popover=auto data-stay-open=true>
      <div popover=hint class=target data-stay-open=true></div>
    </div>
  </div>

  <div> Nested auto/hint ancestors, target is auto
    <div popover=auto class=target data-stay-open=true>
      <div popover=hint data-stay-open=false></div>
    </div>
  </div>

  <div> Unrelated hint, target=hint
    <div popover=auto data-stay-open=true></div>
    <div popover=hint class=target data-stay-open=true></div>
  </div>

  <div> Unrelated hint, target=auto
    <div popover=auto class=target data-stay-open=true></div>
    <div popover=hint data-stay-open=false></div>
  </div>
</div>

<script>
  const tests = Array.from(document.querySelectorAll('#tests>div'));
  runTopLayerTests(tests);
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/popovers/popover-top-layer-nesting-hints.html"
}
```
