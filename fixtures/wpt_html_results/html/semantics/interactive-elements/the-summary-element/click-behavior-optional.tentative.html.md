# html/semantics/interactive-elements/the-summary-element/click-behavior-optional.tentative.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-summary-element/click-behavior-optional.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>summary element: click behavior</title>
<link rel="author" title="Mu-An Chiou" href="mailto:hi@muan.co">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-summary-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<body>
  <div id="log"></div>

  <details id="details">
    <summary id="summary">Summary</summary>
    <p>Contents</p>
  </details>
</body>

<script>
  // This behavior is not specified by HTML standards, but setting focus on
  // clicked summary tag is the current behavior on Chrome, Safari, and Firefox
  // in both Windows and macOS.
  async_test(t => {
    const details = document.getElementById("details")
    const summary = document.getElementById("summary")

    t.step_timeout(() => {
      details.addEventListener("toggle", t.step_func_done(function () {
        assert_equals(details.open, true, "details should be open")
        assert_equals(document.activeElement, summary, "active element should be summary")
        t.done()
      }))

      new test_driver.click(summary)
    }, 200)
  }, "clicking on summary should open details and set focus on summary")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 655,
        "byte_start": 647,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1383,
        "byte_start": 655,
        "col": 9,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1392,
        "byte_start": 1383,
        "col": 1,
        "line": 39
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
  "source_name": "html/semantics/interactive-elements/the-summary-element/click-behavior-optional.tentative.html"
}
```
