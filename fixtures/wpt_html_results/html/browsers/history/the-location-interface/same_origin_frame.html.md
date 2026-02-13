# html/browsers/history/the-location-interface/same_origin_frame.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/same_origin_frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Same-origin subframe for Location cyclic [[Prototype]] test</title>
  <link rel="author" title="Jeff Walden" href="http://whereswalden.com/" />
</head>
<body>
<!-- nothing to do, this window should be accessible to the parent frame -->
<p>Same-origin iframe</p>
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
  "source_name": "html/browsers/history/the-location-interface/same_origin_frame.html"
}
```
