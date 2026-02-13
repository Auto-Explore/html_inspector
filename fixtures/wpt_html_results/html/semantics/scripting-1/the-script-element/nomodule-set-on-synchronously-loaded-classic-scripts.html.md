# html/semantics/scripting-1/the-script-element/nomodule-set-on-synchronously-loaded-classic-scripts.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/nomodule-set-on-synchronously-loaded-classic-scripts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>External classic scripts with nomodule content attribute must not run</title>
<link rel="author" title="Yusuke Suzuki" href="mailto:utatane.tea@gmail.com">
<link rel="author" title="Ryosuke Niwa" href="mailto:rniwa@webkit.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>

window.executed = false;
window.loaded = false;
window.errored = false;
</script>
<script src="./resources/set-script-executed.js" onload="loaded = true" onerror="errored = false"></script>
<script>

test(() => {
    assert_true(executed);
    assert_true(loaded);
    assert_false(errored);
}, 'A synchronously loaded external classic script without nomodule content attribute must run');

window.executed = false;
window.loaded = false;
window.errored = false;
</script>
<script nomodule src="./resources/set-script-executed.js" onload="loaded = true" onerror="errored = false"></script>
<script>

test(() => {
    assert_false(executed);
    assert_false(loaded);
    assert_false(errored);
}, 'A synchronously loaded external classic script with nomodule content attribute must not run');

</script>
</body>
</html>
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
  "source_name": "html/semantics/scripting-1/the-script-element/nomodule-set-on-synchronously-loaded-classic-scripts.html"
}
```
