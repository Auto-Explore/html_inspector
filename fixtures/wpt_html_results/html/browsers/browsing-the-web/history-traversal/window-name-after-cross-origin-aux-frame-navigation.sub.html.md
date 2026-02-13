# html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-aux-frame-navigation.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-aux-frame-navigation.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <!-- window.name should equal "test" after a cross-origin auxiliary frame navigation. -->
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
</head>
<body>
    <script>
        t = async_test("Test that the window name is correct");
        window.addEventListener("message", t.step_func_done(function(e) {
            assert_equals(e.data, true);
        }));
        window.open("support/window-name-navigation.sub.html?hostname={{domains[www1]}}&shouldhavename=true&sendmessage=true");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/window-name-after-cross-origin-aux-frame-navigation.sub.html"
}
```
