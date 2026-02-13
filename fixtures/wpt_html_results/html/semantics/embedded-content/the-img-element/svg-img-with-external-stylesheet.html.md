# html/semantics/embedded-content/the-img-element/svg-img-with-external-stylesheet.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/svg-img-with-external-stylesheet.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>An img element with an svg src should not load external resources from the svg file.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element">
<link rel="match" href="svg-img-with-external-stylesheet-ref.html">
<p>You should see a green square below.</p>
<img width="100" height="100" src="support/external-sheet.svg">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 393,
        "byte_start": 330,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-img-element/svg-img-with-external-stylesheet.html"
}
```
