# html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html

<!doctype html>
<meta charset=utf-8>
<title>Navigation in onload handler</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
  var testFiles = [
    "navigation-in-onload_form-submission-1.html",
    "navigation-in-onload_form-submission-iframe.html",
    "navigation-in-onload_form-submission-dynamic-iframe.html"
  ]

  var t = async_test();

  function scheduleNextTest() {
    setTimeout(runNextTest, 0);
  }

  function runNextTest() {
    var file = testFiles.shift();
    if (!file) {
      t.done();
      return;
    }

    window.open(file);
  }

  function verify(actual, expected, desc) {
    setTimeout(t.step_func(function() {
      assert_equals(actual, expected, desc);
    }), 0);
  }

</script>
<body onload="scheduleNextTest();"></body>
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
  "source_name": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload.html"
}
```
