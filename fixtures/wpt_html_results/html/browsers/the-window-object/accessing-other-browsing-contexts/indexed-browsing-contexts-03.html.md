# html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <meta charset="utf-8">
  <title>HTML Test: indexed property of a Window object</title>
  <link rel="author" title="Intel" href="http://www.intel.com/" />
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script>

  var t1 = async_test("Indexed child browsing contexts");
  function on_load() {
    t1.step(function () {
      assert_equals(window[0], document.getElementsByTagName("object")[0].contentWindow,
                    "The first child browsing context's container should be the object element.");
      assert_equals(window[1], document.getElementsByTagName("iframe")[0].contentWindow,
                    "The second child browsing context's container should be the iframe element.");
    });
    t1.done();
  }

  </script>
</head>
<body onload="on_load()">
  <div id="log"></div>
  <div style="display:none">
    <div id="0"></div>
    <object name="0" type="text/html" data="test2.html"></object>
    <iframe name="0" src="about:blank"></iframe>
  </div>
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
  "source_name": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-03.html"
}
```
