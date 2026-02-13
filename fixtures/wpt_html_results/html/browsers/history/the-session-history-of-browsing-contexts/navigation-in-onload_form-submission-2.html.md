# html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-2.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-2.html",
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
    <title>Navigation in onload handler through form submission</title>
    <script>

      // Verify is called after onload event to ensure history has been stable.
      function verify() {
        // Navigation in onload handler through form submission should not
        // increse history length.
        var runner = window.top.opener;
        runner.verify(history.length, 1,
          "history.length of subtest '" + top.document.title + "'.");
        runner.scheduleNextTest();
        setTimeout(window.close.bind(top), 0);
      }
    </script>
  </head>
  <body onload="setTimeout(verify, 0);">
  </body>
</html>
```

```json
{
  "messages": [],
  "source_name": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-2.html"
}
```
