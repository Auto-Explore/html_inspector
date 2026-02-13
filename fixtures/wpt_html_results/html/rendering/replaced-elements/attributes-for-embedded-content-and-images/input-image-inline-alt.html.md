# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-inline-alt.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-inline-alt.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Input image type fallback content should respect display property.</title>
<meta name="author" title="Yu Han" href="mailto:yuzhehan@chromium.org">
<link rel="match" href="input-image-inline-alt-ref.html">
<link ref="help" href="https://html.spec.whatwg.org/multipage/rendering.html#images-3:represents-5">
<style>
  div {
    border:1px dashed blue;
    font: 1em monospace;
    line-height: 1em;
    height: 100px;
    width: 150px;
  }
  input {
    display: inline;
    font: 1em monospace;
    line-height: 1em;
  }
</style>
<div>
  <input alt="This is a long ALT text which takes up few lines to display." type="image">
  And additional text to be inlined.
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 169,
        "byte_start": 98,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 169,
        "byte_start": 98,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 328,
        "byte_start": 228,
        "col": 1,
        "line": 5
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-inline-alt.html"
}
```
