# html/semantics/scripting-1/the-script-element/nomodule-set-on-external-module-script.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/nomodule-set-on-external-module-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>An external module script with nomodule must run</title>
<link rel="author" title="Yusuke Suzuki" href="mailto:utatane.tea@gmail.com">
<link rel="author" title="Ryosuke Niwa" href="mailto:rniwa@webkit.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script nomodule type="module" src="./resources/exports-cocoa.js"></script>
<script>

waitForLoadEvent = new Promise((resolve) => {
    window.onload = resolve;
});

promise_test(() => {
    return waitForLoadEvent.then(() => {
        assert_equals(typeof cocoa, 'undefined');
        assert_equals(typeof exportedCocoa, 'object');
        assert_equals(exportedCocoa.taste(), 'awesome');
    });
}, 'An external module script with nomodule content attribute must run');

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.module.nomodule.disallowed",
      "message": "A “script” element with a “nomodule” attribute must not have a “type” attribute with the value “module”.",
      "severity": "Warning",
      "span": {
        "byte_end": 431,
        "byte_start": 365,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/scripting-1/the-script-element/nomodule-set-on-external-module-script.html"
}
```
