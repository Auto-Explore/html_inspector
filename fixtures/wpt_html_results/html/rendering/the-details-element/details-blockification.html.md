# html/rendering/the-details-element/details-blockification.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-blockification.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CSS Test: details children blockification</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<meta name="assert" content="Ensure blockification of <details> children">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>

/* continue testing the old behavior that this was intended to test */
details[open]::details-content {
  display: contents;
}

</style>

<div id="example1">
  <details style="display: grid" open>
    <summary style="display: inline">foo</summary>
    <div style="display: inline">bar</span>
  </details>
</div>

<div id="example2" style="display: grid">
  <details style="display: contents" open>
    <summary style="display: inline">foo</summary>
    <div style="display: inline">bar</span>
  </details>
</div>

<script>
  function checkDetails(details) {
    assert_equals(getComputedStyle(details.querySelector('summary')).display, "block");
    assert_equals(getComputedStyle(details.querySelector('div')).display, "block");
  }
  test(() => {
    checkDetails(document.querySelector('#example1'));
    checkDetails(document.querySelector('#example2'));
    assert_equals(getComputedStyle(document.querySelector('#example2>details')).display, "contents");
  }, "Summary and content should have display:block computed value");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “span”.",
      "severity": "Error",
      "span": {
        "byte_end": 739,
        "byte_start": 732,
        "col": 37,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “span”.",
      "severity": "Error",
      "span": {
        "byte_end": 940,
        "byte_start": 933,
        "col": 37,
        "line": 28
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
  "source_name": "html/rendering/the-details-element/details-blockification.html"
}
```
