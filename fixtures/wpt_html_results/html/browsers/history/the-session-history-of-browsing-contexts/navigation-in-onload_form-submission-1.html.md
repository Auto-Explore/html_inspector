# html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-1.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-1.html",
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
      function redirect() {
        document.querySelector("#redirectionForm").submit();
      }
    </script>
  </head>
  <body onload="redirect();">
    <form id="redirectionForm" action="navigation-in-onload_form-submission-2.html" method="get"></form>
  </body>
</html>
```

```json
{
  "messages": [],
  "source_name": "html/browsers/history/the-session-history-of-browsing-contexts/navigation-in-onload_form-submission-1.html"
}
```
