# html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-dynamic-iframe.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-dynamic-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Navigation in onload handler through form submission in a dynamically created iframe</title>
    <script>
      function test() {
        let testFrame = document.createElement("iframe");
        testFrame.src = "navigation-in-onload_form-submission-1.html";
        document.body.appendChild(testFrame);
      }
    </script>
  </head>
  <body onload="test();">
  </body>
</html>
```

```json
{
  "messages": [],
  "source_name": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-dynamic-iframe.html"
}
```
