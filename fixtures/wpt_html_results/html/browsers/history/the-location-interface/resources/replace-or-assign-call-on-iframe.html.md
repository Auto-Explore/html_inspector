# html/browsers/history/the-location-interface/resources/replace-or-assign-call-on-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/resources/replace-or-assign-call-on-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Referer with location.replace and location.assign</title>
  </head>
  <body>
    <iframe src="/resources/blank.html" hidden></iframe>
    <script>
      window.addEventListener('message', function (e) {
        const referrer = e.data;
        window.parent.postMessage(referrer);
      });
      if (window.location.search === "?replace") {
        document.querySelector("iframe").contentWindow.location.replace("iframe-contents.sub.html?replace");
      } else if (window.location.search === "?assign") {
        document.querySelector("iframe").contentWindow.location.assign("iframe-contents.sub.html?assign");
      }
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
  "source_name": "html/browsers/history/the-location-interface/resources/replace-or-assign-call-on-iframe.html"
}
```
