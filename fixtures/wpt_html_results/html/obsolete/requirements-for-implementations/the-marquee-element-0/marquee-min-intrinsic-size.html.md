# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-min-intrinsic-size.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-min-intrinsic-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1692380">
<title>Marquee min intrinsic size should not cause overflow</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel=match href="marquee-min-intrinsic-size-ref.html">
<div style="width:200px; border: 2px solid purple;">
  <div style="display: inline-block;">
    <marquee style="border: 1px solid black; color: transparent;">
Lorem, ipsum dolor sit amet consectetur adipisicing elit. Deserunt commodi
ratione iste tempore nemo mollitia exercitationem error cum excepturi sit ab
eius consectetur quasi possimus facere, iusto est impedit laborum.
    </marquee>
  </div>
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 524,
        "byte_start": 462,
        "col": 5,
        "line": 9
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-min-intrinsic-size.html"
}
```
