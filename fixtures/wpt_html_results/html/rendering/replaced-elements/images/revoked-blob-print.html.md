# html/rendering/replaced-elements/images/revoked-blob-print.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/revoked-blob-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="test-wait">
<title>Printing an image with src="revoked-blob"</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1665343">
<link rel="match" href="revoked-blob-print-ref.html">
<img width=100 height=50>
<script>
  let canvas = document.createElement("canvas");
  canvas.width = 100;
  canvas.height = 50;
  let ctx = canvas.getContext("2d");
  ctx.fillRect(0, 0, 100, 50);
  canvas.toBlob(function(blob) {
    let img = document.querySelector("img");
    let url = URL.createObjectURL(blob);
    img.onload = function() {
      URL.revokeObjectURL(url);
      document.documentElement.className = "";
    };
    img.src = url;
  });
</script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 336,
        "byte_start": 311,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 336,
        "byte_start": 311,
        "col": 1,
        "line": 7
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
  "source_name": "html/rendering/replaced-elements/images/revoked-blob-print.html"
}
```
