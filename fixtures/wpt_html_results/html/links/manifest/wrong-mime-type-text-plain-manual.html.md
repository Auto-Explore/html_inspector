# html/links/manifest/wrong-mime-type-text-plain-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/manifest/wrong-mime-type-text-plain-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test JSON MIME Type support (text/plain)</title>
<link rel="help" href="https://html.spec.whatwg.org/#link-type-manifest" />
<link rel="manifest" href="wrong-mime-type-text-plain.webmanifest.webmanifest" />
<h1>Test JSON MIME Type support for web manifest</h1>
<p>
  To pass, the use agent must treat the manifest as not present. The response's
  Content-Type metadata "text/plain" is <strong>NOT</strong> a JSON MIME type.
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
  "source_name": "html/links/manifest/wrong-mime-type-text-plain-manual.html"
}
```
