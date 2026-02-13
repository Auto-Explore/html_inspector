# html/links/manifest/mime-type-application-manifest+json-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/manifest/mime-type-application-manifest+json-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test JSON MIME Type support (application/manifest+json)</title>
<link rel="help" href="https://html.spec.whatwg.org/#link-type-manifest" />
<link rel="manifest" href="mime-type-application-manifest+json.webmanifest" />
<h1>Test JSON MIME Type support for web manifest</h1>
<p>
  To pass, the use agent must treat the manifest valid (name is "pass"). The
  response's Content-Type metadata is a JSON MIME type
  "application/manifest+json".
</p>
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
  "source_name": "html/links/manifest/mime-type-application-manifest+json-manual.html"
}
```
