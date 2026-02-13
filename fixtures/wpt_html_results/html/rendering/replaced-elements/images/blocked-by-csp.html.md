# html/rendering/replaced-elements/images/blocked-by-csp.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/blocked-by-csp.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Images behave the same when blocked by CSP as when failing to load/broken</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1664156">
<link rel="match" href="blocked-by-csp-ref.html">
<meta http-equiv=content-security-policy content="img-src 'none'">
<style>img { border: solid; }</style>
It should say PASS below:<br>
<img src=image alt="PASS">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.csp.image.blocked",
      "message": "Resource violates Content Security Policy (meta tag): image “image” blocked by “img-src” directive.",
      "severity": "Warning",
      "span": {
        "byte_end": 394,
        "byte_start": 368,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/images/blocked-by-csp.html"
}
```
