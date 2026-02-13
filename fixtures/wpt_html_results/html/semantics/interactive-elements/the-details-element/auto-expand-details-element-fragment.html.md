# html/semantics/interactive-elements/the-details-element/auto-expand-details-element-fragment.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/auto-expand-details-element-fragment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://github.com/whatwg/html/pull/6466">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div style="height:2000px">spacer</div>

<details id=details>
  <div id=target>target</div>
</details>

<script>
async_test(t => {
  assert_false(details.hasAttribute('open'),
    `The <details> should be closed at the start of the test.`);
  assert_equals(window.pageYOffset, 0,
    `The page should be scrolled to the top at the start of the test.`);

  window.location.hash = '#target';

  requestAnimationFrame(t.step_func_done(() => {
    assert_true(details.hasAttribute('open'),
      `<details> should be opened by navigating to an element inside it.`);
    assert_not_equals(window.pageYOffset, 0,
      `The page should be scrolled down to the <details> element.`);
  }));
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 364,
        "byte_start": 354,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/interactive-elements/the-details-element/auto-expand-details-element-fragment.html"
}
```
