# html/semantics/scripting-1/the-script-element/nomodule-set-on-async-classic-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/nomodule-set-on-async-classic-script.html",
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
<!-- Load this script synchronously to ensure test cases below can load it in 200ms -->
<script src="resources/set-script-executed.js"></script>
</head>
<body>
<script>
let supportsNoModule = "noModule" in document.getElementsByTagName("script")[0];

waitForLoadEvent = new Promise((resolve) => {
    window.onload = resolve;
});

promise_test(() => {
    window.executed = false;
    let loaded = false;
    let errored = false;

    let script = document.createElement('script');

    script.src = './resources/set-script-executed.js';
    script.onload = () => loaded = true;
    script.onerror = () => errored = true;
    script.noModule = false;
    document.body.appendChild(script);

    return waitForLoadEvent.then(() => {
        assert_true(supportsNoModule);
        assert_true(executed);
        assert_true(loaded);
        assert_false(errored);
    });
}, 'An asynchronously loaded classic script with noModule set to false must run');

promise_test(() => {
    window.executed = false;
    let loaded = false;
    let errored = false;

    let script = document.createElement('script');
    script.src = './resources/set-script-executed.js';
    script.onload = () => loaded = true;
    script.onerror = () => errored = true;
    script.noModule = true;
    document.body.appendChild(script);

    return waitForLoadEvent.then(() => {
        assert_true(supportsNoModule);
        assert_false(executed);
        assert_false(loaded);
        assert_false(errored);
    });
}, 'An asynchronously loaded classic script with noModule set to true must not run');

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
  "source_name": "html/semantics/scripting-1/the-script-element/nomodule-set-on-async-classic-script.html"
}
```
