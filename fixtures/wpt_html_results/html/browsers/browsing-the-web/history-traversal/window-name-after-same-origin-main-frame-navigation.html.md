# html/browsers/browsing-the-web/history-traversal/window-name-after-same-origin-main-frame-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/window-name-after-same-origin-main-frame-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>window.name after a same-origin main frame navigation</title>
<script src='/resources/testharness.js'></script>
<script src='/resources/testharnessreport.js'></script>
<body>
  <script>
    var win;
    async_test(function(t) {
      win = window.open("support/window-name-after-same-origin-main-frame-navigation-1.sub.html")
      addEventListener("message", t.step_func_done(e => assert_true(e.data)));
    }).add_cleanup(() => {if (win) {win.close()}});
  </script>
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/window-name-after-same-origin-main-frame-navigation.html"
}
```
