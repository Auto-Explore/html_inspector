# html/semantics/embedded-content/media-elements/track/track-element/cors/support/set-cookie.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/cors/support/set-cookie.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Set cookie from location.hash</title>
<script>
if (location.hash)
    document.cookie = decodeURIComponent(location.hash.substr(1))+'=yes;path=/;max-age=15';
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/cors/support/set-cookie.html"
}
```
