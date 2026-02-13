# html/browsers/windows/targeting-with-embedded-null-in-target.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/targeting-with-embedded-null-in-target.html",
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
  <title>Targeting with embedded null in target</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <iframe name="abc">
  </iframe>
  <a href="resources/message-parent.html" target="abc">Click me</a>
  <script>
    var t = async_test();
    var target_name = "abc\u0000def";

    onmessage = t.step_func_done(function (e) {
        assert_equals(e.data.name, target_name,
                      "Should come from a window with the right name");
        assert_equals(e.source, frames[1],
                      "Should come frome the right window");
    });

    t.step(function() {
        var iframe = document.createElement("iframe");
        iframe.setAttribute("name", target_name);
        document.body.appendChild(iframe);
        var a = document.querySelector("a");
        a.target = target_name;
        a.click();
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
  "source_name": "html/browsers/windows/targeting-with-embedded-null-in-target.html"
}
```
