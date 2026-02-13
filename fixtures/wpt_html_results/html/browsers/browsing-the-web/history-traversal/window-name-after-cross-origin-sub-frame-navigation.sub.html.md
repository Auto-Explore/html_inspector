# html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-sub-frame-navigation.sub.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-sub-frame-navigation.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <!-- window.name should equal "test" after a cross-origin sub frame navigation. -->
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
</head>
<body>
    <script>
        t = async_test("Test that the window name is correct");
        window.addEventListener("message", t.step_func_done(function(e) {
            assert_equals(e.data, true);
        }));
    </script>
    <iframe src="support/window-name-navigation.sub.html?hostname={{domains[www1]}}&shouldhavename=true&sendmessage=true";
    </iframe>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.lt_expecting_attr_name",
      "message": "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 470,
        "col": 5,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": null
    },
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-sub-frame-navigation.sub.html"
}
```
