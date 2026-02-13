# html/browsers/windows/browsing-context-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>HTML Test: Newly-Created browsing context Window and `this`</title>
    <link rel="author" title="Intel" href="http://www.intel.com/" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/get-host-info.sub.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>

    /**
     * Test for creating a new browsing context, [Browsing Contexts](#windows).
     * This test is separate from the rest of the browsing-content-creation tests
     * because the `iframe` has a src and thus its document won't be `about:blank`.
     */
    var doc, iframe;

    setup(function () {
      iframe = document.createElement("iframe");
      iframe.src = get_host_info().HTTP_REMOTE_ORIGIN + "/html/browsers/windows/resources/browsing-context-window.html";
      document.body.appendChild(iframe);
      doc = iframe.contentDocument;
    });

    async_test(function (t) {
      window.onmessage = t.step_func(function (e) {
        assert_equals(e.data.thisWindowEquivalency, true, "The global `this` for the created browsing context should be a reference to Window through WindowProxy");
        t.done();
      });
    });

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
  "source_name": "html/browsers/windows/browsing-context-window.html"
}
```
