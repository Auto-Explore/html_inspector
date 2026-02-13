# html/semantics/selectors/pseudo-classes/dir-dynamic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/dir-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <link rel="author" title="Vincent Hilla" href="mailto:vhilla@mozilla.com">
  <link rel="help" href="https://html.spec.whatwg.org/#the-directionality">
</head>
<body>
  <input id="inp"/>
  <textarea id="ta"></textarea>
  <div id="div"></div>
  <pre id="pre"></pre>

  <script>
    function doTest(e) {
      e.dir = "ltr";
      assert_true(e.matches(":dir(ltr)"), "dir to ltr on " + e.tagName + " element");

      e.dir = "rtl";
      assert_true(e.matches(":dir(rtl)"), "dir to rtl on " + e.tagName + " element");

      e.dir = "auto";
      assert_true(e.matches(":dir(ltr)"), "dir to auto, empty text on " + e.tagName + " element");

      e.value = "\u05D0;";
      e.textContent = "\u05D0;";
      assert_true(e.matches(":dir(rtl)"), "auto dir, text to Hebrew on " + e.tagName + " element");

      e.dir = "ltr";
      assert_true(e.matches(":dir(ltr)"), "dir to ltr, Hebrew text on " + e.tagName + " element");

      e.dir = "auto";
      assert_true(e.matches(":dir(rtl)"), "dir to auto, Hebrew text on " + e.tagName + " element");

      e.removeAttribute("dir");
      assert_true(e.matches(":dir(ltr)"), "dir removed, Hebrew text on " + e.tagName + " element");
    }

    const elements = [inp, ta, div, pre];
    for (const e of elements) {
      test(() => doTest(e), "Dynamically changing dir, text on " + e.tagName.toLowerCase() + " element");
    }
  </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/selectors/pseudo-classes/dir-dynamic.html"
}
```
