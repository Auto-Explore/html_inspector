# html/meta/refresh-time.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/meta/refresh-time.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test fractional values in meta http-equiv=refresh</title>
<link rel="author" title="Psychpsyo"  href="mailto:psychpsyo@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/#pragma-directives">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
  async function measureRefreshTime(src) {
    const frame = document.createElement("iframe");
    document.body.appendChild(frame);

    const loadPromise = new Promise(resolve => {
      frame.addEventListener("load", () => {
        resolve(performance.now());
      });
    });
    frame.src = src;
    const startTime = await loadPromise;

    const unloadPromise = new Promise(resolve => {
      frame.contentWindow.addEventListener("beforeunload", () => {
        resolve(performance.now());
      });
    });
    const endTime = await unloadPromise;
    return endTime - startTime;
  }

  promise_test(async test => {
    const refreshTime = await measureRefreshTime("resources/refresh1.html");
    assert_greater_than(refreshTime, 900);
  }, "Ensure that refresh is observed");

  promise_test(async test => {
    const refreshTime = await measureRefreshTime("resources/refresh1.99.html");
    assert_greater_than(refreshTime, 900);
  }, "Ensure that non-fractional part in refresh time does not get discarded");

  promise_test(async test => {
    const refreshTime = await measureRefreshTime("resources/refresh1dotdot9dot.html");
    assert_greater_than(refreshTime, 900);
  }, "Ensure that multiple periods in refresh time just get ignored");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/meta/refresh-time.html"
}
```
