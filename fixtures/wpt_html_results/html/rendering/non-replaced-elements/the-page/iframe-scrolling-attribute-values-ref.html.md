# html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The scrolling attribute</title>
<link rel="author" href="mailto:masonf@chromium.org">



<style>
    iframe {
        width: 100px;
        height: 100px;
    }
</style>

<p>This page tests the behavior of the <tt>scrolling</tt> attribute on
<tt>&lt;iframe&gt;</tt> elements which contain a page large enough to need to
be scrolled.</p>

<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="no"></iframe>
<iframe src="support/big-page.html" scrolling="no"></iframe>
<iframe src="support/big-page.html" scrolling="no"></iframe>
<iframe src="support/big-page.html" scrolling="no"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>
<iframe src="support/big-page.html" scrolling="auto"></iframe>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.tt.obsolete",
      "message": "The “tt” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 237,
        "byte_start": 233,
        "col": 40,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.element.tt.obsolete",
      "message": "The “tt” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 269,
        "byte_start": 265,
        "col": 1,
        "line": 15
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
  "source_name": "html/rendering/non-replaced-elements/the-page/iframe-scrolling-attribute-values-ref.html"
}
```
