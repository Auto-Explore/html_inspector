# html/semantics/scripting-1/the-script-element/nomodule-set-on-inline-classic-scripts.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/nomodule-set-on-inline-classic-scripts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Inline classic scripts with nomodule content attribute must not run</title>
<link rel="author" title="Yusuke Suzuki" href="mailto:utatane.tea@gmail.com">
<link rel="author" title="Ryosuke Niwa" href="mailto:rniwa@webkit.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
window.executed = true;
</script>
<script>

test(() => {
    assert_true(executed);
}, 'An inline classic script without nomodule content attribute must run');


window.executed = false;
</script>
<script nomodule>
window.executed = true;
</script>
<script>

test(() => {
    assert_false(executed);
}, 'An inline classic script with nomodule content attribute must not run');

</script>
<script>

test(() => {
    window.executed = false;
    const element = document.createElement("script");
    element.noModule = false;
    element.textContent = `window.executed = true`;
    document.body.appendChild(element);
    assert_true(window.executed);
}, 'An inline classic script element dynamically inserted after noModule was set to false must run.');

test(() => {
    window.executed = false;
    const element = document.createElement("script");
    element.noModule = true;
    element.textContent = `window.executed = true`;
    document.body.appendChild(element);
    assert_false(window.executed);
}, 'An inline classic script element dynamically inserted after noModule was set to true must not run.');

window.executed = false;
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
  "source_name": "html/semantics/scripting-1/the-script-element/nomodule-set-on-inline-classic-scripts.html"
}
```
